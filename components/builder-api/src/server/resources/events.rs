use crate::{db::models::channel::{AuditPackage,
                                  AuditPackageEvent,
                                  ListEvents},
            server::{authorize::authorize_session,
                     error::{Error,
                             Result},
                     framework::headers,
                     helpers::{self,
                               req_state,
                               Pagination}}};
use actix_web::{http,
                web::{self,
                      Query,
                      ServiceConfig},
                HttpRequest,
                HttpResponse};

pub struct Events {}

impl Events {
    // Route registration
    //
    pub fn register(cfg: &mut ServiceConfig) {
        cfg.route("/depot/events", web::get().to(get_events));
    }
}

#[allow(clippy::needless_pass_by_value)]
fn get_events(req: HttpRequest, pagination: Query<Pagination>) -> HttpResponse {
    match do_get_events(&req, &pagination) {
        Ok((events, count)) => postprocess_event_list(&req, &events, count, &pagination),
        Err(err) => {
            debug!("{}", err);
            err.into()
        }
    }
}

fn do_get_events(req: &HttpRequest,
                 pagination: &Query<Pagination>)
                 -> Result<(Vec<AuditPackageEvent>, i64)> {
    let opt_session_id = match authorize_session(req, None, None) {
        Ok(session) => Some(session.get_id()),
        Err(_) => None,
    };

    let (page, per_page) = helpers::extract_pagination_in_pages(pagination);

    let conn = req_state(req).db.get_conn().map_err(Error::DbError)?;

    let el = ListEvents { page:  page as i64,
                          limit: per_page as i64, };

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
