//! SigmaOS-productivity: productivity component model.

use std::collections::BTreeMap;
use std::fmt;

/// Primary domain entity for productivity component.
#[derive(Debug, Clone, PartialEq)]
pub struct ProductivityItem {
    pub id: String,
    pub name: String,
    pub item_type: String,
    pub enabled: bool,
}

/// Secondary domain entity for productivity component.
#[derive(Debug, Clone, PartialEq)]
pub struct ProductivityConfig {
    pub id: String,
    pub value: String,
    pub description: String,
}

/// Errors for the productivity component domain.
#[derive(Debug, PartialEq)]
pub enum ProductivityError {
    EmptyName,
    Duplicate(String),
    NotFound(String),
}

impl fmt::Display for ProductivityError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::EmptyName => write!(f, "name cannot be empty"),
            Self::Duplicate(id) => write!(f, "duplicate id: {id}"),
            Self::NotFound(id) => write!(f, "not found: {id}"),
        }
    }
}

impl std::error::Error for ProductivityError {}

/// Registry and manager for productivity component.
pub struct ProductivityManager {
    pub name: String,
    items: BTreeMap<String, ProductivityItem>,
    configs: BTreeMap<String, ProductivityConfig>,
}

impl ProductivityManager {
    /// Create a new manager instance.
    pub fn new(name: &str) -> Self {
        Self {
            name: name.to_string(),
            items: BTreeMap::new(),
            configs: BTreeMap::new(),
        }
    }

    /// Add a primary item with fail-fast validation.
    pub fn add_item(&mut self, item: ProductivityItem) -> Result<(), ProductivityError> {
        if item.name.is_empty() {
            return Err(ProductivityError::EmptyName);
        }
        if self.items.contains_key(&item.id) {
            return Err(ProductivityError::Duplicate(item.id.clone()));
        }
        self.items.insert(item.id.clone(), item);
        Ok(())
    }

    /// Retrieve a primary item by id.
    pub fn get_item(&self, id: &str) -> Option<&ProductivityItem> {
        self.items.get(id)
    }

    /// Count of primary items.
    pub fn count(&self) -> usize {
        self.items.len()
    }

    /// Add a secondary configuration entry.
    pub fn add_config(&mut self, cfg: ProductivityConfig) {
        self.configs.insert(cfg.id.clone(), cfg);
    }

    /// Count of configuration entries.
    pub fn config_count(&self) -> usize {
        self.configs.len()
    }

    /// Enable or disable an item.
    pub fn set_enabled(&mut self, id: &str, enabled: bool) -> Result<(), ProductivityError> {
        self.items
            .get_mut(id)
            .map(|item| item.enabled = enabled)
            .ok_or_else(|| ProductivityError::NotFound(id.to_string()))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn make_productivity_item(id: &str, name: &str) -> ProductivityItem {
        ProductivityItem {
            id: id.to_string(),
            name: name.to_string(),
            item_type: "productivity-default".to_string(),
            enabled: true,
        }
    }

    fn make_productivity_config(id: &str, val: &str) -> ProductivityConfig {
        ProductivityConfig {
            id: id.to_string(),
            value: val.to_string(),
            description: format!("{} config", id),
        }
    }

    #[test]
    fn test_create_aaa() {
        let mgr = ProductivityManager::new("test");
        assert_eq!(mgr.name, "test");
        assert_eq!(mgr.count(), 0);
        assert_eq!(mgr.config_count(), 0);
    }

    #[test]
    fn test_add_item_aaa() {
        let mut mgr = ProductivityManager::new("test");
        let item = make_productivity_item("item1", "First Item");
        assert!(mgr.add_item(item).is_ok());
        assert_eq!(mgr.count(), 1);
    }

    #[test]
    fn test_empty_name_rejected_aaa() {
        let mut mgr = ProductivityManager::new("test");
        let item = make_productivity_item("item1", "");
        assert_eq!(mgr.add_item(item), Err(ProductivityError::EmptyName));
    }

    #[test]
    fn test_duplicate_rejected_aaa() {
        let mut mgr = ProductivityManager::new("test");
        let item1 = make_productivity_item("item1", "First");
        let item2 = make_productivity_item("item1", "Duplicate");
        mgr.add_item(item1).unwrap();
        assert_eq!(mgr.add_item(item2), Err(ProductivityError::Duplicate("item1".to_string())));
    }

    #[test]
    fn test_config_addition_aaa() {
        let mut mgr = ProductivityManager::new("test");
        let cfg = make_productivity_config("cfg1", "value1");
        mgr.add_config(cfg);
        assert_eq!(mgr.config_count(), 1);
    }

    #[test]
    fn test_get_item_aaa() {
        let mut mgr = ProductivityManager::new("test");
        let item = make_productivity_item("item1", "First Item");
        mgr.add_item(item).unwrap();
        assert!(mgr.get_item("item1").is_some());
        assert!(mgr.get_item("none").is_none());
    }

    #[test]
    fn test_set_enabled_aaa() {
        let mut mgr = ProductivityManager::new("test");
        let item = make_productivity_item("item1", "First Item");
        mgr.add_item(item).unwrap();
        assert!(mgr.set_enabled("item1", false).is_ok());
        assert!(!mgr.get_item("item1").unwrap().enabled);
        assert_eq!(mgr.set_enabled("none", true), Err(ProductivityError::NotFound("none".to_string())));
    }
}
