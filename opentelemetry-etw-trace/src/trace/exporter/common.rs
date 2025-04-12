//! Common utilities and constants for ETW exporter.

use std::collections::HashMap;
use opentelemetry::KeyValue;

/// Default schema version.
pub const SCHEMA_VERSION: &str = "1.0.0";

/// Default provider name if none is provided.
pub const DEFAULT_PROVIDER_NAME: &str = "OpenTelemetryEtwTraceProvider";

/// Converts a HashMap to a JSON string.
///
/// # Arguments
///
/// * `map` - The HashMap to convert.
pub fn hashmap_to_json(map: &HashMap<String, String>) -> String {
    todo!("Implement hashmap_to_json")
}

/// Generate a unique GUID from a string.
///
/// # Arguments
///
/// * `name` - The string to generate a GUID from.
pub fn generate_provider_guid(name: &str) -> uuid::Uuid {
    todo!("Implement generate_provider_guid")
}

/// Generates ETW event descriptor with specific characteristics.
pub fn create_event_descriptor() -> u8 {
    todo!("Implement create_event_descriptor")
}