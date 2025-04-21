//! Reentrant span processor implementation for ETW.

use opentelemetry::Context;
use opentelemetry_sdk::{trace::{SpanExporter, SpanProcessor}, Resource};

use super::exporter::ETWExporter;

/// A reentrant span processor that exports spans to ETW.
/// This processor is designed to be reentrant, which means it can handle
/// nested spans and avoid deadlocks.
#[derive(Debug)]
pub struct ReentrantSpanProcessor {
    event_exporter: ETWExporter,
}

impl ReentrantSpanProcessor {
    /// constructor
    pub fn new(provider_name: &str) -> Self {
        let exporter = ETWExporter::new(provider_name);
        ReentrantSpanProcessor {
            event_exporter: exporter,
        }
    }
}

impl SpanProcessor for ReentrantSpanProcessor {
    fn on_start(&self, _span: &mut opentelemetry_sdk::trace::Span, _cx: &Context) {
        // No action needed on start.
    }

    fn on_end(&self, span: opentelemetry_sdk::trace::SpanData) {
        let _ = futures_executor::block_on(self.event_exporter.export(vec![span]));
    }

    fn force_flush(&self) -> opentelemetry_sdk::error::OTelSdkResult {
        Ok(())
    }

    fn shutdown(&self) -> opentelemetry_sdk::error::OTelSdkResult {
        todo!()
        // This does not work due to &mut self and &self conflict.
        //self.event_exporter.shutdown()
    }

    fn set_resource(&mut self, _resource: &Resource) {
        self.event_exporter.set_resource(_resource);
    }

}