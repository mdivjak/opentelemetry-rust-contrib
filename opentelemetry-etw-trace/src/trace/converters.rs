//! Utility functions for converting OpenTelemetry types to ETW format.

use opentelemetry::{
    trace::{SpanKind, Status, SpanContext},
    KeyValue,
};
use std::collections::HashMap;

/// Converts a SpanKind to a string representation.
///
/// # Arguments
///
/// * `kind` - The SpanKind to convert.
#[warn(dead_code)]
pub fn span_kind_to_string(_kind: SpanKind) -> &'static str {
    todo!("Implement span_kind_to_string")
}

/// Converts a Status to a string representation.
///
/// # Arguments
///
/// * `status` - The Status to convert.
#[warn(dead_code)]
pub fn status_to_string(_status: Status) -> &'static str {
    todo!("Implement status_to_string")
}

/// Converts attributes to a HashMap.
///
/// # Arguments
///
/// * `attributes` - The attributes to convert.
#[warn(dead_code)]
pub fn attributes_to_hashmap(_attributes: &[KeyValue]) -> HashMap<String, String> {
    todo!("Implement attributes_to_hashmap")
}

/// Converts a SpanContext to trace and span IDs.
///
/// # Arguments
///
/// * `context` - The SpanContext to convert.
#[warn(dead_code)]
pub fn extract_trace_span_ids(_context: &SpanContext) -> (String, String) {
    todo!("Implement extract_trace_span_ids")
}

/// Converts a timestamp to a format suitable for ETW.
///
/// # Arguments
///
/// * `timestamp` - The timestamp to convert.
#[warn(dead_code)]
pub fn timestamp_to_etw_format(_timestamp: std::time::SystemTime) -> u64 {
    todo!("Implement timestamp_to_etw_format")
}