//! ETW Span Exporter implementation.

use opentelemetry::{
    trace::{SpanContext, SpanKind, Status},
    Key, KeyValue,
};
use std::{collections::HashMap, sync::Mutex};

mod common;
mod part_a;
mod part_b;
mod part_c;

use common::*;
use part_a::PartA;
use part_b::PartB;
use part_c::PartC;

/// A span exporter that writes spans to ETW.
#[derive(Debug)]
pub struct ETWSpanExporter {
    provider_name: String,
    provider_guid: uuid::Uuid,
    resource_attributes: Vec<KeyValue>,
    // ETW provider handle would be stored here
    provider_handle: Mutex<Option<u64>>,
}

impl ETWSpanExporter {
    /// Creates a new ETW span exporter.
    ///
    /// # Arguments
    ///
    /// * `provider_name` - The name of the ETW provider to use.
    /// * `resource_attributes` - Resource attributes to include in the spans.
    pub fn new(provider_name: &str, resource_attributes: Vec<KeyValue>) -> Self {
        todo!("Implement ETWSpanExporter::new")
    }

    /// Registers the ETW provider.
    fn register_provider(&self) -> Result<u64, ETWExporterError> {
        todo!("Implement register_provider")
    }

    /// Exports a span to ETW.
    ///
    /// # Arguments
    ///
    /// * `span_context` - The context of the span to export.
    /// * `span_name` - The name of the span.
    /// * `span_kind` - The kind of the span.
    /// * `status` - The status of the span.
    /// * `start_time` - The start time of the span.
    /// * `end_time` - The end time of the span.
    /// * `attributes` - The attributes of the span.
    /// * `parent_span_id` - The ID of the parent span, if any.
    pub fn export_span(
        &self,
        span_context: &SpanContext,
        span_name: &str,
        span_kind: SpanKind,
        status: Status,
        start_time: std::time::SystemTime,
        end_time: std::time::SystemTime,
        attributes: &[KeyValue],
        parent_span_id: Option<String>,
    ) -> Result<(), ETWExporterError> {
        todo!("Implement export_span")
    }
}

/// Errors that can occur when exporting spans to ETW.
#[derive(Debug, thiserror::Error)]
pub enum ETWExporterError {
    /// Failed to register ETW provider.
    #[error("Failed to register ETW provider: {0}")]
    ProviderRegistrationError(String),

    /// Failed to write ETW event.
    #[error("Failed to write ETW event: {0}")]
    EventWriteError(String),

    /// Other error.
    #[error("Other error: {0}")]
    Other(String),
}