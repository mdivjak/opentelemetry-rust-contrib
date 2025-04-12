//! Common utilities for ETW exporting.

use opentelemetry::Value;
use opentelemetry_sdk::{Resource, InstrumentationLibrary};
use std::collections::HashMap;

/// Helper structure to extract and format OpenTelemetry resource attributes for ETW events.
#[derive(Debug, Clone)]
pub struct ResourceAttributes {
    service_name: Option<String>,
    service_namespace: Option<String>,
    service_instance_id: Option<String>,
    service_version: Option<String>,
    attributes: HashMap<String, Value>,
}

impl ResourceAttributes {
    /// Create ResourceAttributes from an OpenTelemetry Resource.
    pub fn from_resource(resource: &Resource) -> Self {
        let mut result = Self {
            service_name: None,
            service_namespace: None,
            service_instance_id: None,
            service_version: None,
            attributes: HashMap::new(),
        };

        // Extract standard resource attributes defined by OpenTelemetry
        for (key, value) in resource.iter() {
            match key.as_str() {
                "service.name" => {
                    if let Value::String(val) = value {
                        result.service_name = Some(val.to_string());
                    }
                }
                "service.namespace" => {
                    if let Value::String(val) = value {
                        result.service_namespace = Some(val.to_string());
                    }
                }
                "service.instance.id" => {
                    if let Value::String(val) = value {
                        result.service_instance_id = Some(val.to_string());
                    }
                }
                "service.version" => {
                    if let Value::String(val) = value {
                        result.service_version = Some(val.to_string());
                    }
                }
                _ => {
                    result.attributes.insert(key.to_string(), value.clone());
                }
            }
        }

        result
    }

    /// Get the service name or a default value.
    pub fn service_name(&self) -> &str {
        self.service_name.as_deref().unwrap_or("unknown_service")
    }

    /// Get the service namespace if available.
    pub fn service_namespace(&self) -> Option<&str> {
        self.service_namespace.as_deref()
    }

    /// Get the service instance ID if available.
    pub fn service_instance_id(&self) -> Option<&str> {
        self.service_instance_id.as_deref()
    }

    /// Get the service version if available.
    pub fn service_version(&self) -> Option<&str> {
        self.service_version.as_deref()
    }

    /// Get all additional resource attributes.
    pub fn attributes(&self) -> &HashMap<String, Value> {
        &self.attributes
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use opentelemetry_sdk::Resource;
    use std::collections::HashMap;

    #[test]
    fn test_resource_attributes_extraction() {
        let attributes = [
            ("service.name", "test_service"),
            ("service.namespace", "test_namespace"),
            ("service.instance.id", "test_instance"),
            ("service.version", "1.0.0"),
            ("custom.attribute", "custom_value"),
        ]
        .iter()
        .map(|(k, v)| (k.to_string(), Value::String(v.to_string())))
        .collect::<HashMap<_, _>>();

        let resource = Resource::from(attributes);
        let resource_attrs = ResourceAttributes::from_resource(&resource);

        assert_eq!(resource_attrs.service_name(), "test_service");
        assert_eq!(resource_attrs.service_namespace(), Some("test_namespace"));
        assert_eq!(resource_attrs.service_instance_id(), Some("test_instance"));
        assert_eq!(resource_attrs.service_version(), Some("1.0.0"));
        
        let custom_attr = resource_attrs.attributes().get("custom.attribute");
        assert!(custom_attr.is_some());
        assert_eq!(custom_attr.unwrap().as_str(), Some("custom_value"));
    }
}