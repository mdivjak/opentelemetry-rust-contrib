//! ETW trace exporter implementation.

mod exporter;
pub use exporter::ETWExporter;

use opentelemetry::trace::TraceError;
use opentelemetry_sdk::trace::SpanData;
use std::sync::Arc;
use tracelogging_dynamic as tld;

/// ETW exporter for OpenTelemetry traces.
/// 
/// This exporter sends trace data to Windows ETW (Event Tracing for Windows).
#[derive(Debug)]
pub struct ETWExporter {
    provider: Arc<tld::Provider>,
    resource: opentelemetry_sdk::Resource,
}

impl ETWExporter {
    /// Create a new ETW exporter with the specified provider name.
    pub fn new(provider_name: &str) -> Result<Self, TraceError> {
        let provider = tld::Builder::new()
            .build(provider_name)
            .map_err(|e| TraceError::from(format!("Failed to create ETW provider: {}", e)))?;

        Ok(Self {
            provider: Arc::new(provider),
            resource: opentelemetry_sdk::Resource::empty(),
        })
    }
}

impl Drop for ETWExporter {
    fn drop(&mut self) {
        // Provider will be closed automatically by Drop
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create_etw_exporter() {
        let result = ETWExporter::new("OTelETWTraceTest");
        assert!(result.is_ok());
    }
}