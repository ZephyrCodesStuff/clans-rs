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