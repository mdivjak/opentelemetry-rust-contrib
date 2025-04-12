//! Implementation of a reentrant-safe span processor for ETW exports.

use opentelemetry::trace::{TraceError, TraceResult};
use opentelemetry_sdk::trace::{SpanData, SpanExporter, SpanProcessor};
use std::sync::{Arc, Mutex};

/// A span processor that ensures export operations are reentrant-safe.
///
/// This processor is designed to prevent deadlocks that can occur when ETW
/// logging during the export process triggers additional spans to be created.
#[derive(Debug)]
pub struct ReentrantSpanProcessor<T: SpanExporter> {
    exporter: Arc<Mutex<T>>,
    in_export: std::sync::atomic::AtomicBool,
}

impl<T: SpanExporter> ReentrantSpanProcessor<T> {
    /// Create a new reentrant span processor with the given exporter.
    pub fn new(exporter: T) -> Self {
        Self {
            exporter: Arc::new(Mutex::new(exporter)),
            in_export: std::sync::atomic::AtomicBool::new(false),
        }
    }
}

impl<T: SpanExporter> SpanProcessor for ReentrantSpanProcessor<T> {
    fn on_start(&self, _span: &mut opentelemetry_sdk::trace::Span, _parent_context: &opentelemetry::Context) {
        // No action needed on start
    }

    fn on_end(&self, span: SpanData) {
        // Avoid reentrant calls by checking if we're already exporting
        if self.in_export.load(std::sync::atomic::Ordering::Relaxed) {
            return;
        }

        // Set the in_export flag to prevent reentrant calls
        self.in_export.store(true, std::sync::atomic::Ordering::Relaxed);

        // Export the span
        let result = futures_executor::block_on(async {
            if let Ok(exporter) = self.exporter.lock() {
                exporter.export(vec![span]).await
            } else {
                // If we can't acquire the lock, the exporter is likely being used by another thread
                Err(TraceError::from("Failed to acquire exporter lock"))
            }
        });

        if let Err(e) = result {
            // Log the error (using println since we can't use the trace system here)
            println!("Error exporting span: {}", e);
        }

        // Reset the in_export flag
        self.in_export.store(false, std::sync::atomic::Ordering::Relaxed);
    }

    fn force_flush(&self) -> TraceResult<()> {
        // Skip if we're already exporting to avoid reentrant issues
        if self.in_export.load(std::sync::atomic::Ordering::Relaxed) {
            return Ok(());
        }

        if let Ok(exporter) = self.exporter.lock() {
            exporter.force_flush()
        } else {
            Err(TraceError::from("Failed to acquire exporter lock during flush"))
        }
    }

    fn shutdown(&self) -> TraceResult<()> {
        if let Ok(exporter) = self.exporter.lock() {
            exporter.shutdown()
        } else {
            Err(TraceError::from("Failed to acquire exporter lock during shutdown"))
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use opentelemetry::trace::TraceResult;
    use opentelemetry_sdk::trace::{SpanData, SpanExporter};
    use std::sync::atomic::{AtomicUsize, Ordering};
    use std::sync::Arc;

    // Mock span exporter for testing
    #[derive(Debug)]
    struct MockExporter {
        export_count: Arc<AtomicUsize>,
    }

    impl MockExporter {
        fn new() -> Self {
            Self {
                export_count: Arc::new(AtomicUsize::new(0)),
            }
        }

        fn export_count(&self) -> usize {
            self.export_count.load(Ordering::SeqCst)
        }
    }

    impl SpanExporter for MockExporter {
        fn export(&self, _spans: Vec<SpanData>) -> TraceResult<()> {
            self.export_count.fetch_add(1, Ordering::SeqCst);
            Ok(())
        }

        fn shutdown(&self) -> TraceResult<()> {
            Ok(())
        }

        fn force_flush(&self) -> TraceResult<()> {
            Ok(())
        }
    }

    #[test]
    fn test_reentrant_processor() {
        let exporter = MockExporter::new();
        let processor = ReentrantSpanProcessor::new(exporter);

        // Create a dummy span for testing
        let span_data = SpanData::new(
            opentelemetry_sdk::trace::SpanContext::empty_context(),
            opentelemetry::Context::current(),
            "test_span".to_string(),
            opentelemetry::trace::SpanKind::Internal,
            "".to_string(),
            opentelemetry::trace::Status::Unset,
            opentelemetry::trace::StatusCode::Unset,
            opentelemetry::KeyValue::new("", ""),
            vec![],
            vec![],
            opentelemetry_sdk::trace::span::SpanEvents::default(),
            opentelemetry_sdk::trace::span::SpanLinks::default(),
            opentelemetry::trace::StatusCode::Unset,
            std::time::SystemTime::now(),
            std::time::SystemTime::now(),
            None,
            None,
        );

        processor.on_end(span_data);

        // Get the lock and check that the export was called
        let exporter = processor.exporter.lock().unwrap();
        assert_eq!(exporter.export_count(), 1);

        // Check flush and shutdown
        assert!(processor.force_flush().is_ok());
        assert!(processor.shutdown().is_ok());
    }
}