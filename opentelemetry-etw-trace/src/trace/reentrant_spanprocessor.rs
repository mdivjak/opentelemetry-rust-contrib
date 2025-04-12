//! Reentrant span processor implementation for ETW.

use opentelemetry::Context;
use opentelemetry_sdk::trace::SpanProcessor;
use std::sync::Arc;

use crate::trace::ETWSpanExporter;

/// A reentrant span processor that exports spans to ETW.
/// This processor is designed to be reentrant, which means it can handle
/// nested spans and avoid deadlocks.
#[derive(Debug)]
pub struct ReentrantSpanProcessor {
    _exporter: Arc<ETWSpanExporter>,
}

impl ReentrantSpanProcessor {
    /// Creates a new reentrant span processor with the given exporter.
    ///
    /// # Arguments
    ///
    /// * `exporter` - The ETW span exporter to use.
    pub fn new(_exporter: Arc<ETWSpanExporter>) -> Self {
        todo!("Implement ReentrantSpanProcessor::new")
    }
}

impl SpanProcessor for ReentrantSpanProcessor {
    fn on_start(&self, _span: &mut opentelemetry_sdk::trace::Span, _cx: &Context) {
        todo!()
    }

    fn on_end(&self, _span: opentelemetry_sdk::trace::SpanData) {
        todo!()
    }

    fn force_flush(&self) -> opentelemetry_sdk::error::OTelSdkResult {
        todo!()
    }

    fn shutdown(&self) -> opentelemetry_sdk::error::OTelSdkResult {
        todo!()
    }
}