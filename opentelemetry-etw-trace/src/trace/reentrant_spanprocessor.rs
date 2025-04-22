//! Reentrant span processor implementation for ETW.

use std::sync::{Arc, Mutex};

use opentelemetry::Context;
use opentelemetry_sdk::{trace::{SpanExporter, SpanProcessor}, Resource};

use super::exporter::ETWExporter;

/// A reentrant span processor that exports spans to ETW.
/// This processor is designed to be reentrant, which means it can handle
/// nested spans and avoid deadlocks.
#[derive(Debug)]
pub struct ReentrantSpanProcessor {
    event_exporter: Arc<Mutex<ETWExporter>>,
}

impl ReentrantSpanProcessor {
    /// constructor
    pub fn new(provider_name: &str) -> Self {
        let exporter = ETWExporter::new(provider_name);
        ReentrantSpanProcessor {
            event_exporter: Arc::new(Mutex::new(exporter)),
        }
    }
}

impl SpanProcessor for ReentrantSpanProcessor {
    fn on_start(&self, _span: &mut opentelemetry_sdk::trace::Span, _cx: &Context) {
        // No action needed on start.
    }

    fn on_end(&self, span: opentelemetry_sdk::trace::SpanData) {
        if let Ok(exporter) = self.event_exporter.lock() {
            let _ = futures_executor::block_on(exporter.export(vec![span]));
        }
    }

    fn force_flush(&self) -> opentelemetry_sdk::error::OTelSdkResult {
        if let Ok(mut exporter) = self.event_exporter.lock() {
            exporter.force_flush()
        } else {
            Ok(())
        }
    }

    fn shutdown(&self) -> opentelemetry_sdk::error::OTelSdkResult {
        if let Ok(mut exporter) = self.event_exporter.lock() {
            exporter.shutdown()
        } else {
            Ok(())
        }
    }

    fn set_resource(&mut self, _resource: &Resource) {
        if let Ok(mut exporter) = self.event_exporter.lock() {
            exporter.set_resource(_resource);
        }
    }

}