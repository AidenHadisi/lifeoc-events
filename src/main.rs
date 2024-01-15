#![allow(async_fn_in_trait)]

use cms::{Cms, WordPress};
use futures::future::try_join_all;
use lambda_http::{lambda_runtime::streaming::Body, service_fn, Request, Response};
use std::env;
use tracing::error;

pub mod cms;
pub mod event;
pub mod parser;

/// The error type used by the library using thiserror.
#[derive(Debug, thiserror::Error)]
pub enum Error {
    /// The email could not be parsed.
    #[error("The email could not be parsed.")]
    ParseError,

    /// The API returned an error.
    #[error("The API returned an error: {0}")]
    Api(String),
}

/// The result type used by the library.
pub type Result<T> = std::result::Result<T, Error>;

#[tokio::main]
async fn main() -> Result<()> {
    // required to enable CloudWatch error logging by the runtime
    tracing_subscriber::fmt()
        .with_max_level(tracing::Level::INFO)
        // disable printing the name of the module in every log line.
        .with_target(false)
        // this needs to be set to false, otherwise ANSI color codes will
        // show up in a confusing manner in CloudWatch logs.
        .with_ansi(false)
        // disabling time is handy because CloudWatch will add the ingestion time.
        .without_time()
        .init();

    lambda_http::run(service_fn(function_handler))
        .await
        .unwrap();

    Ok(())
}

/// The function handler for the Lambda function.
async fn function_handler(event: Request) -> Result<Response<Body>> {
    let api = WordPress::new(
        &env::var("WP_USERNAME").expect("WP_USERNAME not set"),
        &env::var("WP_PASSWORD").expect("WP_PASSWORD not set"),
    );

    let body = std::str::from_utf8(event.body().as_ref()).unwrap();
    let events = parser::parse_email(body);

    // save all the events concurrently
    try_join_all(events.iter().map(|event| api.save_event(event))).await?;

    Ok(Response::builder()
        .status(200)
        .body(Body::from("Success!"))
        .unwrap())
}
