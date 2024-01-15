//! Contains the service for interacting with the CMS.

use crate::event::Event;
use crate::{Error, Result};
use base64::prelude::BASE64_STANDARD;
use base64::Engine;
use reqwest::header::{HeaderMap, HeaderValue, AUTHORIZATION};
use reqwest::Client;

/// The CMS service.
pub trait Cms {
    /// Returns the events from the CMS.
    async fn save_event(&self, event: &Event) -> crate::Result<()>;
}

/// WordPress CMS.
pub struct WordPress {
    /// The HTTP client.
    client: Client,
}

impl WordPress {
    /// Creates a new WPEvents.
    pub fn new(username: &str, password: &str) -> Self {
        let credentials = BASE64_STANDARD.encode(format!("{username}:{password}"));

        // Create the authorization header value
        let auth_header_value = HeaderValue::from_str(&format!("Basic {}", credentials)).unwrap();

        let mut headers = HeaderMap::new();
        headers.insert(AUTHORIZATION, auth_header_value);

        let client = Client::builder().default_headers(headers).build().unwrap();
        Self { client }
    }
}

impl Cms for WordPress {
    /// Saves the event to the CMS.
    async fn save_event(&self, event: &Event) -> Result<()> {
        self.client
            .post("https://lifeoc.org/wp-json/tribe/events/v1/events")
            .json(event)
            .send()
            .await
            .map_err(|e| Error::Api(e.to_string()))?
            .error_for_status()
            .map_err(|e| {
                Error::Api(format!(
                    "Failed to create event with status code: {}",
                    e.status().unwrap_or_default()
                ))
            })?;

        Ok(())
    }
}
