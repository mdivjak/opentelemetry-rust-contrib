//! Extension trait for adding ETW exporter to a TracerProvider.

use opentelemetry_sdk::trace::TracerProviderBuilder;
use super::reentrant_spanprocessor::ReentrantSpanProcessor;

/// Extension trait for adding ETW exporter to a TracerProvider.
pub trait ETWTracerProviderBuilderExt {
    /// Adds an ETW exporter to a TracerProvider.
    fn with_etw_exporter(self, provider_name: &str) -> Self;
}

impl ETWTracerProviderBuilderExt for TracerProviderBuilder {
    fn with_etw_exporter(self, provider_name: &str) -> Self {
        let reentrant_processor = ReentrantSpanProcessor::new(provider_name);
        self.with_span_processor(reentrant_processor)
    }
}