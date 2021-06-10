use crate::{db::models::channel::{AuditPackage,
                                  AuditPackageEvent,
                                  ListEvents},
            server::{authorize::authorize_session,
                     error::{Error,
                             Result},
                     framework::headers,
                     helpers::{self,
                               req_state,
                               Pagination,
                               ToChannel}}};
use actix_web::{http::{self,
                       HeaderMap},
                web::{self,
                      Query,
                      ServiceConfig},
                HttpRequest,
                HttpResponse};
use builder_core::http_client::{HttpClient,
                                USER_AGENT_BLDR};

pub struct Events {}

impl Events {
    // Route registration
    //
    pub fn register(cfg: &mut ServiceConfig) {
        cfg.route("/depot/events", web::get().to(get_events));
    }
}

#[allow(clippy::needless_pass_by_value)]
async fn get_events(req: HttpRequest,
                    pagination: Query<Pagination>,
                    channel: Query<ToChannel>)
                    -> HttpResponse {
    let headers = req.headers();
    if check_request_is_from_on_prem(headers) {
        return get_events_from_saas_builder(headers).await;
    }

    match do_get_events(&req, &pagination, &channel) {
        Ok((events, count)) => postprocess_event_list(&req, &events, count, &pagination),
        Err(err) => {
            debug!("{}", err);
            err.into()
        }
    }
}

fn do_get_events(req: &HttpRequest,
                 pagination: &Query<Pagination>,
                 channel: &Query<ToChannel>)
                 -> Result<(Vec<AuditPackageEvent>, i64)> {
    let opt_session_id = match authorize_session(req, None, None) {
        Ok(session) => Some(session.get_id() as i64),
        Err(_) => None,
    };
    let (page, per_page) = helpers::extract_pagination_in_pages(pagination);

    let conn = req_state(req).db.get_conn().map_err(Error::DbError)?;

    let el = ListEvents { page:       page as i64,
                          limit:      per_page as i64,
                          account_id: opt_session_id,
                          channel:    channel.channel.trim().to_string(), };
    match AuditPackage::list(el, &*conn).map_err(Error::DieselError) {
        Ok((packages, count)) => {
            let pkg_events: Vec<AuditPackageEvent> =
                packages.into_iter().map(|p| p.into()).collect();

            Ok((pkg_events, count))
        }
        Err(e) => Err(e),
    }
}

pub fn postprocess_event_list(_req: &HttpRequest,
                              events: &[AuditPackageEvent],
                              count: i64,
                              pagination: &Query<Pagination>)
                              -> HttpResponse {
    let (start, _) = helpers::extract_pagination(pagination);
    let event_count = events.len() as isize;
    let stop = match event_count {
        0 => count,
        _ => (start + event_count - 1) as i64,
    };

    debug!("postprocessing event list, start: {}, stop: {}, total_count: {}",
           start, stop, count);

    let body =
        helpers::package_results_json(&events, count as isize, start as isize, stop as isize);

    let mut response = if count as isize > (stop as isize + 1) {
        HttpResponse::PartialContent()
    } else {
        HttpResponse::Ok()
    };

    response.header(http::header::CONTENT_TYPE, headers::APPLICATION_JSON)
            .header(http::header::CACHE_CONTROL, headers::NO_CACHE)
            .body(body)
}

fn check_request_is_from_on_prem(headers: &HeaderMap) -> bool {
    if let Some(ref referer) = headers.get(http::header::HOST) {
        if let Ok(s) = referer.to_str() {
            if s.contains("habitat.sh") {
                return false;
            }
        }
    }

    true
}

// Invoke the REST API on the SaaS builder
async fn get_events_from_saas_builder(map: &HeaderMap) -> HttpResponse {
    let mut headers = reqwest::header::HeaderMap::new();
    headers.insert(USER_AGENT_BLDR.0.clone(), USER_AGENT_BLDR.1.clone());
    if map.contains_key(http::header::AUTHORIZATION) {
        headers.insert(http::header::AUTHORIZATION,
                       map.get(http::header::AUTHORIZATION).unwrap().clone());
    }

    let http_client = match HttpClient::new("https://bldr.acceptance.habitat.sh", headers) {
        Ok(client) => client,
        Err(err) => {
            debug!("HttpClient Error: {:?}", err);
            return HttpResponse::InternalServerError().body(err.to_string());
        }
    };

    match http_client.get("https://bldr.acceptance.habitat.sh/v1/depot/events?range=0")
                     .send()
                     .await
                     .map_err(Error::BuilderCore)
    {
        Ok(response) => {
            match response.text().await {
                Ok(body) => {
                    let mut http_response = HttpResponse::Ok();

                    http_response.header(http::header::CONTENT_TYPE, headers::APPLICATION_JSON)
                                 .header(http::header::CACHE_CONTROL, headers::NO_CACHE)
                                 .body(body)
                }
                Err(err) => {
                    debug!("Error getting response text: {:?}", err);
                    HttpResponse::InternalServerError().body(err.to_string())
                }
            }
        }
        Err(err) => {
            debug!("Error sending request: {:?}", err);
            HttpResponse::InternalServerError().body(err.to_string())
        }
    }
}
