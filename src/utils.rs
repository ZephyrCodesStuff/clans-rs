//! This module contains utility functions that are used in the application,
//! scoped by their purpose.

pub mod date_format {
    //! Custom date formatting for serialization with ``chrono`` and ``serde``.
    
    use chrono::{DateTime, Utc};

    /// ISO 8601 format (yyyy-mm-ddThh:mm:ssZ).
    pub fn iso8601(dt: &DateTime<Utc>) -> String {
        dt.format("%Y-%m-%dT%H:%M:%SZ").to_string()
    }
}

pub mod xml_format {
    //! Custom XML formatting for serialization with ``serde``.

    /// Trait for serializing a type to an XML element.
    pub trait ToXML {
        /// Serialize the string to an XML element.
        fn to_xml(&self) -> String;
    }
}

#[cfg(feature = "admin")]
pub mod auth {
    //! Helpers for authenticating Admin requests.

    use actix_web::{body::MessageBody, dev::{ServiceRequest, ServiceResponse}, middleware::Next, Error};

    /// Header name for the Admin token.
    const ADMIN_TOKEN_HEADER: &str = "X-Admin-Token";

    /// Environment variable name for the Admin token.
    const ADMIN_TOKEN_ENV: &str = "ADMIN_TOKEN";
    
    /// Middleware that checks for the `X-Admin-Token` header.
    #[allow(clippy::future_not_send)]
    pub async fn admin(req: ServiceRequest,
        next: Next<impl MessageBody>,
    ) -> Result<ServiceResponse<impl MessageBody>, Error> {
        // Check if the request path starts with `/admin` and skip authentication if it doesn't.
        if !req.path().starts_with("/admin") {
            return next.call(req).await;
        }

        let token = req.headers().get(ADMIN_TOKEN_HEADER)
            .ok_or_else(|| actix_web::error::ErrorUnauthorized("Missing authorization"))?;
        
        let env_token = std::env::var(ADMIN_TOKEN_ENV)
            .map_err(|_| {
                log::warn!("An Admin endpoint was called, but the server is missing an ADMIN_TOKEN environment variable.");
                
                actix_web::error::ErrorInternalServerError("Internal server error")
            })?;

        let Ok(token_str) = token.to_str() else {
            return Err(actix_web::error::ErrorBadRequest("Invalid token"));
        };

        if token_str != env_token {
            return Err(actix_web::error::ErrorUnauthorized("Invalid authorization"));
        }

        next.call(req).await
    }
}