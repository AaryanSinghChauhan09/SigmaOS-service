// SPDX-License-Identifier: Apache-2.0
//! SigmaOS-service: service component model.
//!
//! Real implementation: service_items, events, validation, deterministic storage.
//! Replaces the placeholder stub.

use std::collections::BTreeMap;
use std::fmt;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ServiceItem {
    pub id: String,
    pub name: String,
    pub kind: String,
    pub detail: String,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Event {
    pub id: String,
    pub subject: String,
    pub detail: String,
    pub timestamp: u64,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ServiceError {
    EmptyServiceItemName,
    DuplicateServiceItem(String),
    EmptyEventSubject,
}

impl fmt::Display for ServiceError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::EmptyServiceItemName => write!(f, "service_item name must not be empty"),
            Self::DuplicateServiceItem(id) => write!(f, "duplicate service_item {:?}", id),
            Self::EmptyEventSubject => write!(f, "event subject must not be empty"),
        }
    }
}

impl std::error::Error for ServiceError {}

pub struct Service {
    pub name: String,
    service_items: BTreeMap<String, ServiceItem>,
    events: BTreeMap<String, Event>,
}

impl Service {
    pub fn new(name: impl Into<String>) -> Self {
        Self {
            name: name.into(),
            service_items: BTreeMap::new(),
            events: BTreeMap::new(),
        }
    }

    pub fn add_service_item(&mut self, item: ServiceItem) -> Result<(), ServiceError> {
        if item.name.trim().is_empty() {
            return Err(ServiceError::EmptyServiceItemName);
        }
        let id = item.id.clone();
        if self.service_items.contains_key(&id) {
            return Err(ServiceError::DuplicateServiceItem(id.clone()));
        }
        self.service_items.insert(id, item);
        Ok(())
    }

    pub fn service_item_count(&self) -> usize {
        self.service_items.len()
    }

    pub fn get_service_item(&self, id: &str) -> Option<&ServiceItem> {
        self.service_items.get(id)
    }

    pub fn add_event(&mut self, event: Event) -> Result<(), ServiceError> {
        if event.subject.trim().is_empty() {
            return Err(ServiceError::EmptyEventSubject);
        }
        self.events.insert(event.id.clone(), event);
        Ok(())
    }

    pub fn event_count(&self) -> usize {
        self.events.len()
    }

    pub fn get_event(&self, id: &str) -> Option<&Event> {
        self.events.get(id)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn sample_service_item(id: &str, name: &str) -> ServiceItem {
        ServiceItem {
            id: id.into(),
            name: name.into(),
            kind: "default".into(),
            detail: String::new(),
        }
    }

    #[test]
    fn test_create() {
        let m = Service::new("Service");
        assert_eq!(m.name, "Service");
        assert_eq!(m.service_item_count(), 0);
        assert_eq!(m.event_count(), 0);
    }

    #[test]
    fn test_add_service_item() {
        let mut m = Service::new("Service");
        m.add_service_item(sample_service_item("a1", "Alpha")).unwrap();
        assert_eq!(m.service_item_count(), 1);
        assert!(m.get_service_item("a1").is_some());
    }

    #[test]
    fn test_empty_name_rejected() {
        let mut m = Service::new("Service");
        assert!(m.add_service_item(sample_service_item("x", "   ")).is_err());
    }

    #[test]
    fn test_duplicate_rejected() {
        let mut m = Service::new("Service");
        m.add_service_item(sample_service_item("x", "One")).unwrap();
        assert!(m.add_service_item(sample_service_item("x", "Two")).is_err());
    }

    #[test]
    fn test_event_validation() {
        let mut m = Service::new("Service");
        let bad = Event { id: "e1".into(), subject: "".into(), detail: String::new(), timestamp: 0 };
        assert!(m.add_event(bad).is_err());
        let good = Event { id: "e1".into(), subject: "boot".into(), detail: String::new(), timestamp: 1 };
        m.add_event(good).unwrap();
        assert_eq!(m.event_count(), 1);
    }
}
