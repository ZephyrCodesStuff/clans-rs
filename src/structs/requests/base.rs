//! Base request structure and Extractor implementation.

use std::fmt::Debug;

use actix_web::{web::Buf, FromRequest};
use serde::Deserialize;

/// Generic wrapper for a request.
#[derive(Debug, Deserialize)]
pub struct Request<T> {
    /// Root element of the request.
    #[serde(rename = "$value")]
    pub request: T,
}

impl<'a, T: Deserialize<'a> + Debug> FromRequest for Request<T> {
    type Error = actix_web::Error;
    type Future = std::pin::Pin<Box<dyn std::future::Future<Output = Result<Self, Self::Error>> + 'static>>;
    
    /// Get the request body from the client and deserialize it.
    fn from_request(req: &actix_web::HttpRequest, payload: &mut actix_web::dev::Payload) -> Self::Future {
        let fut = actix_web::web::Bytes::from_request(req, payload);
        Box::pin(async move {
            let bytes = fut.await?;

            // Parse the XML
            let request = serde_xml_rs::from_reader(bytes.reader())
                .map_err(actix_web::error::ErrorInternalServerError)?;
            
            // DEBUG: print the XML's contents
            log::debug!("Request: {:#?}", request);            

            Ok(Self { request })
        })
    }
}