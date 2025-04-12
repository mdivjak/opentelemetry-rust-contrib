//! Part B of the ETW event structure for trace spans.
//!
//! Part B contains span-specific data like name, kind, and status.

use opentelemetry::trace::{SpanKind, Status};
use serde::Serialize;

/// Part B of the ETW event structure for trace spans.
#[derive(Debug, Serialize)]
pub struct PartB {
    /// Name of the span.
    pub name: String,
    
    /// Kind of the span (server, client, producer, consumer, internal).
    pub kind: String,
    
    /// Status of the span (ok, error).
    pub status: String,
    
    /// Status message if status is error.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status_message: Option<String>,
    
    /// Start time of the span.
    pub start_time: String,
    
    /// End time of the span.
    pub end_time: String,
    
    /// Duration of the span in nanoseconds.
    pub duration_ns: u64,
}

impl PartB {
    /// Creates a new Part B with the given span data.
    ///
    /// # Arguments
    ///
    /// * `span_name` - The name of the span.
    /// * `span_kind` - The kind of the span.
    /// * `status` - The status of the span.
    /// * `start_time` - The start time of the span.
    /// * `end_time` - The end time of the span.
    pub fn new(
        span_name: &str,
        span_kind: SpanKind,
        status: Status,
        start_time: std::time::SystemTime,
        end_time: std::time::SystemTime,
    ) -> Self {
        todo!("Implement PartB::new")
    }
}