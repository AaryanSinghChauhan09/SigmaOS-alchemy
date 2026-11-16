// SPDX-License-Identifier: Apache-2.0
//! SigmaOS-alchemy: alchemy component model.
//!
//! Real implementation: alchemy_items, events, validation, deterministic storage.
//! Replaces the placeholder stub.

use std::collections::BTreeMap;
use std::fmt;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AlchemyItem {
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
pub enum AlchemyError {
    EmptyAlchemyItemName,
    DuplicateAlchemyItem(String),
    EmptyEventSubject,
}

impl fmt::Display for AlchemyError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::EmptyAlchemyItemName => write!(f, "alchemy_item name must not be empty"),
            Self::DuplicateAlchemyItem(id) => write!(f, "duplicate alchemy_item {:?}", id),
            Self::EmptyEventSubject => write!(f, "event subject must not be empty"),
        }
    }
}

impl std::error::Error for AlchemyError {}

pub struct Alchemy {
    pub name: String,
    alchemy_items: BTreeMap<String, AlchemyItem>,
    events: BTreeMap<String, Event>,
}

impl Alchemy {
    pub fn new(name: impl Into<String>) -> Self {
        Self {
            name: name.into(),
            alchemy_items: BTreeMap::new(),
            events: BTreeMap::new(),
        }
    }

    pub fn add_alchemy_item(&mut self, item: AlchemyItem) -> Result<(), AlchemyError> {
        if item.name.trim().is_empty() {
            return Err(AlchemyError::EmptyAlchemyItemName);
        }
        let id = item.id.clone();
        if self.alchemy_items.contains_key(&id) {
            return Err(AlchemyError::DuplicateAlchemyItem(id.clone()));
        }
        self.alchemy_items.insert(id, item);
        Ok(())
    }

    pub fn alchemy_item_count(&self) -> usize {
        self.alchemy_items.len()
    }

    pub fn get_alchemy_item(&self, id: &str) -> Option<&AlchemyItem> {
        self.alchemy_items.get(id)
    }

    pub fn add_event(&mut self, event: Event) -> Result<(), AlchemyError> {
        if event.subject.trim().is_empty() {
            return Err(AlchemyError::EmptyEventSubject);
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

    fn sample_alchemy_item(id: &str, name: &str) -> AlchemyItem {
        AlchemyItem {
            id: id.into(),
            name: name.into(),
            kind: "default".into(),
            detail: String::new(),
        }
    }

    #[test]
    fn test_create() {
        let m = Alchemy::new("Alchemy");
        assert_eq!(m.name, "Alchemy");
        assert_eq!(m.alchemy_item_count(), 0);
        assert_eq!(m.event_count(), 0);
    }

    #[test]
    fn test_add_alchemy_item() {
        let mut m = Alchemy::new("Alchemy");
        m.add_alchemy_item(sample_alchemy_item("a1", "Alpha")).unwrap();
        assert_eq!(m.alchemy_item_count(), 1);
        assert!(m.get_alchemy_item("a1").is_some());
    }

    #[test]
    fn test_empty_name_rejected() {
        let mut m = Alchemy::new("Alchemy");
        assert!(m.add_alchemy_item(sample_alchemy_item("x", "   ")).is_err());
    }

    #[test]
    fn test_duplicate_rejected() {
        let mut m = Alchemy::new("Alchemy");
        m.add_alchemy_item(sample_alchemy_item("x", "One")).unwrap();
        assert!(m.add_alchemy_item(sample_alchemy_item("x", "Two")).is_err());
    }

    #[test]
    fn test_event_validation() {
        let mut m = Alchemy::new("Alchemy");
        let bad = Event { id: "e1".into(), subject: "".into(), detail: String::new(), timestamp: 0 };
        assert!(m.add_event(bad).is_err());
        let good = Event { id: "e1".into(), subject: "boot".into(), detail: String::new(), timestamp: 1 };
        m.add_event(good).unwrap();
        assert_eq!(m.event_count(), 1);
    }
}
