//! This module contains utility functions that are used in the application,
//! scoped by their purpose.

pub mod xml_format {
    //! Custom XML formatting for serialization with ``serde``.

    /// Trait for serializing a type to an XML element.
    pub trait ToXml {
        /// Serialize the string to an XML element.
        fn to_xml(&self, name: Option<&str>) -> String;
    }
}