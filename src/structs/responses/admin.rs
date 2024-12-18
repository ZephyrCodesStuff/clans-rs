//! Response structs for the Admin API endpoints.

use actix_web::{body::BoxBody, HttpResponse, Responder};
use serde::Serialize;

use super::error::ErrorCode;

/// Base response structure for the Admin API.
#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Response {
    /// The status of the request.
    pub status_code: u8,
}

impl From<u8> for Response {
    fn from(code: u8) -> Self {
        Self {
            status_code: code,
        }
    }
}

impl From<ErrorCode> for Response {
    fn from(code: ErrorCode) -> Self {
        Self {
            status_code: code as u8,
        }
    }
}

impl From<Response> for HttpResponse {
    fn from(response: Response) -> Self {
        Self::Ok().json(response)
    }
}

impl Responder for Response {
    type Body = BoxBody;

    fn respond_to(self, _req: &actix_web::HttpRequest) -> actix_web::HttpResponse {
        self.into()
    }
}