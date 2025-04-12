//! Part C of the ETW event structure for trace spans.
//!
//! Part C contains custom attributes from the span.

use opentelemetry::KeyValue;
use serde::Serialize;
use std::collections::HashMap;

/// Part C of the ETW event structure for trace spans.
/// This contains custom attributes from the span.
#[derive(Debug, Serialize)]
pub struct PartC {
    /// Custom attributes from the span.
    #[serde(flatten)]
    pub attributes: HashMap<String, String>,
}

impl PartC {
    /// Creates a new Part C with the given span attributes.
    ///
    /// # Arguments
    ///
    /// * `attributes` - The attributes of the span.
    pub fn new(attributes: &[KeyValue]) -> Self {
        todo!("Implement PartC::new")
    }
}