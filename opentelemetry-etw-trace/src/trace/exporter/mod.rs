//! Implementation details of the ETW exporter.

use super::common::ResourceAttributes;
use opentelemetry::trace::TraceError;
use opentelemetry_sdk::trace::{self, SpanData, SpanExporter};
use std::sync::Arc;
use tracelogging_dynamic as tld;

/// ETW exporter for OpenTelemetry traces.
pub(crate) struct ETWExporter {
    provider: Arc<tld::Provider>,
    resource: opentelemetry_sdk::Resource,
}

impl ETWExporter {
    /// Create a new ETW exporter with the specified provider name.
    pub fn new(provider_name: &str, resource: opentelemetry_sdk::Resource) -> Result<Self, TraceError> {
        let provider = tld::Builder::new()
            .build(provider_name)
            .map_err(|e| TraceError::from(format!("Failed to create ETW provider: {}", e)))?;

        Ok(Self {
            provider: Arc::new(provider),
            resource,
        })
    }
}

impl SpanExporter for ETWExporter {
    fn export(&self, spans: Vec<SpanData>) -> opentelemetry::trace::Result<()> {
        // Placeholder for the actual export implementation
        // Will be expanded in later implementation tasks
        for span in spans {
            // Basic logging to show the export happened (temporary for initial setup)
            println!("Exporting span: {}", span.name);
        }
        Ok(())
    }

    fn shutdown(&self) -> opentelemetry::trace::Result<()> {
        // Nothing to do here, provider will be closed when dropped
        Ok(())
    }

    fn force_flush(&self) -> opentelemetry::trace::Result<()> {
        // ETW writes immediately, no need to flush
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use opentelemetry_sdk::{Resource, trace::SpanData};

    #[test]
    fn test_export_spans() {
        let exporter = ETWExporter::new(
            "OTelETWTraceTest",
            Resource::empty(),
        ).unwrap();
        
        // This is a minimal test to ensure the export function doesn't panic
        // More comprehensive tests will be added as implementation progresses
        let result = exporter.export(Vec::new());
        assert!(result.is_ok());
    }
}