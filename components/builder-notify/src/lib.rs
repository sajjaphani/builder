extern crate features;
#[macro_use]
extern crate log;
#[macro_use]
extern crate serde_derive;

pub mod cli;
pub mod config;
pub mod error;
pub mod hook;
pub mod hub;
pub mod server;
pub mod webhook_client;

use crate::{config::Config,
            hook::Webhook,
            hub::Hub,
            webhook_client::WebhookClient};

pub fn get_hub(config: &Config) -> Hub {
    debug!("NotifyConfig {:?}", config);
    let mut hub = Hub::new();
    let webhooks = &config.hub.webhooks;
    for webhook in webhooks {
        hub.add(Webhook { endpoint: webhook.endpoint.clone(),
                          client:   WebhookClient::new().unwrap(), });
    }

    hub
}
