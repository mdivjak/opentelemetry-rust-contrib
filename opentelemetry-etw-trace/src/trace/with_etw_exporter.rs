//! Extension trait for TracerProvider to easily add ETW exporter.

use crate::trace::{ETWExporter, ReentrantSpanProcessor};
use opentelemetry::trace::TraceError;
use opentelemetry_sdk::trace::TracerProviderBuilder;

/// Extension trait that adds methods to TracerProviderBuilder for ETW exporter configuration.
pub trait ETWTracerProviderBuilderExt {
    /// Add an ETW exporter to the tracer provider with the specified provider name.
    ///
    /// # Arguments
    ///
    /// * `provider_name` - The name for the ETW provider that will be registered in Windows ETW.
    ///
    /// # Returns
    ///
    /// The modified builder instance for further configuration.
    ///
    /// # Example
    ///
    /// ```no_run
    /// use opentelemetry_etw_trace::trace::ETWTracerProviderBuilderExt;
    /// use opentelemetry_sdk::trace::TracerProvider;
    ///
    /// let provider = TracerProvider::builder()
    ///     .with_etw_exporter("MyAppTraces")
    ///     .build();
    /// ```
    fn with_etw_exporter(self, provider_name: &str) -> Self;
}

impl ETWTracerProviderBuilderExt for TracerProviderBuilder {
    fn with_etw_exporter(self, provider_name: &str) -> Self {
        // Attempt to create the ETW exporter
        match ETWExporter::new(provider_name) {
            Ok(exporter) => {
                // Wrap with ReentrantSpanProcessor for safety
                let processor = ReentrantSpanProcessor::new(exporter);
                self.with_span_processor(processor)
            }
            Err(err) => {
                // Log the error and return the builder unmodified
                eprintln!("Failed to create ETW exporter: {}", err);
                self
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use opentelemetry_sdk::trace::TracerProvider;

    #[test]
    fn test_with_etw_exporter() {
        let provider = TracerProvider::builder()
            .with_etw_exporter("TestETWTraces")
            .build();
        
        // Basic test to ensure the builder completes without error
        assert!(provider.tracer("test").span_builder("test_span").start_with_context(&opentelemetry::Context::current()).is_recording());
    }
}