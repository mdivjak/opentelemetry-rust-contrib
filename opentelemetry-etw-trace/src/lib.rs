//! OpenTelemetry ETW Trace Exporter
//!
//! This crate provides an OpenTelemetry trace exporter that sends trace data to 
//! Windows Event Tracing for Windows (ETW).
//!
//! # Example
//!
//! ```no_run
//! use opentelemetry_etw_trace::trace::ETWTracerProviderBuilderExt;
//! use opentelemetry_sdk::trace::TracerProvider;
//!
//! // Create a new tracer provider with ETW exporter
//! let provider = TracerProvider::builder()
//!     .with_etw_exporter("MyAppTraces")
//!     .build();
//!
//! // Get a tracer from the provider
//! let tracer = provider.tracer("my_component");
//! ```

pub mod trace;