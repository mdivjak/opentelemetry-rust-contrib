//! Part A of the ETW event structure for trace spans.
//!
//! Part A contains the core data for the ETW event, including
//! timestamp, trace ID, span ID, and resource attributes.

use opentelemetry::{KeyValue, trace::SpanContext};
use serde::Serialize;
use std::collections::HashMap;

/// Part A of the ETW event structure for trace spans.
#[derive(Debug, Serialize)]
pub struct PartA {
    /// Schema version for the ETW event.
    pub ver: String,
    
    /// Timestamp of the event.
    pub time: String,
    
    /// Service name from resource attributes.
    pub name: String,
    
    /// Service version from resource attributes.
    pub sv: String,
    
    /// Trace ID from span context.
    pub trace_id: String,
    
    /// Span ID from span context.
    pub span_id: String,
    
    /// Parent span ID if available.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parent_id: Option<String>,
}

impl PartA {
    /// Creates a new Part A with the given span context, timestamp, and resource attributes.
    ///
    /// # Arguments
    ///
    /// * `span_context` - The context of the span.
    /// * `timestamp` - The timestamp of the event.
    /// * `parent_span_id` - The ID of the parent span, if any.
    /// * `resource_attributes` - Resource attributes from the tracer provider.
    pub fn new(
        span_context: &SpanContext,
        timestamp: std::time::SystemTime,
        parent_span_id: Option<String>,
        resource_attributes: &[KeyValue],
    ) -> Self {
        todo!("Implement PartA::new")
    }
}