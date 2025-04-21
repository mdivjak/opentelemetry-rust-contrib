use std::{pin::Pin, sync::Arc};
use opentelemetry::Key;
use opentelemetry_sdk::{error::OTelSdkError, trace::{SpanData, SpanExporter}};
use tracelogging_dynamic as tld;
use std::fmt::Debug;

mod part_a;
mod part_b;
mod part_c;

#[derive(Default)]
struct Resource {
    pub cloud_role: Option<String>,
    pub cloud_role_instance: Option<String>,
}

pub(crate) struct ETWExporter {
    provider: Pin<Arc<tld::Provider>>,
    resource: Resource,
}

fn enabled_callback_noop(
    _source_id: &tld::Guid,
    _event_control_code: u32,
    _level: tld::Level,
    _match_any_keyword: u64,
    _match_all_keyword: u64,
    _filter_data: usize,
    _callback_context: usize,
) {
    // Unused callback.
}

impl ETWExporter {
    const KEYWORD: u64 = 1;

    pub(crate) fn new(provider_name: &str) -> Self {
        let mut options = tld::Provider::options();

        options.callback(enabled_callback_noop, 0x0);
        let provider = Arc::pin(tld::Provider::new(provider_name, &options));
        // SAFETY: tracelogging (ETW) enables an ETW callback into the provider when `register()` is called.
        // This might crash if the provider is dropped without calling unregister before.
        // This only affects static providers.
        // On dynamically created providers, the lifetime of the provider is tied to the object itself, so `unregister()` is called when dropped.
        unsafe {
            provider.as_ref().register();
        }

        ETWExporter {
            provider,
            resource: Default::default(),
        }
    }

    fn enabled(&self, level: tld::Level) -> bool {
        // On unit tests, we skip this check to be able to test the exporter as no provider is active.
        if cfg!(test) {
            return true;
        }

        self.provider.enabled(level, Self::KEYWORD)
    }

    pub(crate) fn export_trace_data(
        &self,
        _span: Vec<SpanData>,
    ) -> opentelemetry_sdk::error::OTelSdkResult {
        todo!("implement trace data export");
    }
}

impl Debug for ETWExporter {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str("ETW trace exporter")
    }
}

impl SpanExporter for ETWExporter {
    async fn export(
        &self,
        batch: Vec<opentelemetry_sdk::trace::SpanData>,
    ) -> Result<(), opentelemetry_sdk::error::OTelSdkError> {
        self.export_trace_data(batch)
    }
    
    fn shutdown(&mut self) -> opentelemetry_sdk::error::OTelSdkResult {
        let res = self.provider.as_ref().unregister();
        if res != 0 {
            return Err(OTelSdkError::InternalFailure(format!(
                "Failed to unregister provider. Win32 error: {res}"
            )));
        }
        Ok(())
    }
    
    fn force_flush(&mut self) -> opentelemetry_sdk::error::OTelSdkResult {
        todo!()
    }
    
    fn set_resource(&mut self, resource: &opentelemetry_sdk::Resource) {
        self.resource.cloud_role = resource
            .get(&Key::from_static_str("service.name"))
            .map(|v| v.to_string());
        self.resource.cloud_role_instance = resource
            .get(&Key::from_static_str("service.instance.id"))
            .map(|v| v.to_string());
    }
}