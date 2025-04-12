//! The ETW exporter will enable applications to use OpenTelemetry API
//! to capture the telemetry trace events, and write to the ETW subsystem.

#![warn(missing_debug_implementations, missing_docs)]

mod trace;

pub use trace::*;