//! Extension trait for adding ETW exporter to a TracerProvider.

use opentelemetry::KeyValue;
use opentelemetry_sdk::trace::TracerProviderBuilder;


/// Extension trait for adding ETW exporter to a TracerProvider.
pub trait ETWTracerProviderBuilderExt: Sized {
    /// Adds an ETW exporter to a TracerProvider.
    ///
    /// # Arguments
    ///
    /// * `provider_name` - The name of the ETW provider to use.
    fn with_etw_exporter(self, provider_name: &str) -> Self;

    /// Adds an ETW exporter with additional resource attributes to a TracerProvider.
    ///
    /// # Arguments
    ///
    /// * `provider_name` - The name of the ETW provider to use.
    /// * `resource_attributes` - Additional resource attributes to include in the spans.
    fn with_etw_exporter_and_resource(
        self,
        provider_name: &str, 
        resource_attributes: Vec<KeyValue>,
    ) -> Self;
}

impl ETWTracerProviderBuilderExt for TracerProviderBuilder {
    fn with_etw_exporter(self, _provider_name: &str) -> Self {
        todo!("Implement ETW exporter creation and attachment to the TracerProvider")
    }

    fn with_etw_exporter_and_resource(
        self,
        _provider_name: &str, 
        _resource_attributes: Vec<KeyValue>,
    ) -> Self {
        todo!("Implement ETW exporter with resource attributes")
    }
}