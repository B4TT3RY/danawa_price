use std::{collections::HashMap, path::Path};

use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Default, Copy, Clone)]
pub struct PriceData {
    pub card_price: Option<i32>,
    pub cash_price: Option<i32>,
}

#[derive(Serialize, Deserialize, Default)]
pub struct PriceStorage {
    price_map: HashMap<String, PriceData>,
}

#[derive(Debug)]
pub enum StorageError {
    Io(std::io::Error),
    Serialize(toml::ser::Error),
    Deserialize(toml::de::Error),
}

impl PriceStorage {
    pub fn load(path: impl AsRef<Path>) -> Result<Self, StorageError> {
        let data = std::fs::read(path)?;
        let storage = toml::from_slice(&data)?;
        Ok(Self { price_map: storage })
    }

    pub fn save(&self, path: impl AsRef<Path>) -> Result<(), StorageError> {
        let data = toml::to_vec(&self.price_map)?;
        std::fs::write(path, data)?;
        Ok(())
    }

    pub fn get(&self, code: &str) -> PriceData {
        self.price_map
            .get(code)
            .copied()
            .unwrap_or_else(Default::default)
    }

    pub fn insert(&mut self, code: String, new_data: PriceData) {
        self.price_map.insert(code, new_data);
    }
}

impl From<std::io::Error> for StorageError {
    fn from(e: std::io::Error) -> Self {
        Self::Io(e)
    }
}

impl From<toml::ser::Error> for StorageError {
    fn from(e: toml::ser::Error) -> Self {
        Self::Serialize(e)
    }
}

impl From<toml::de::Error> for StorageError {
    fn from(e: toml::de::Error) -> Self {
        Self::Deserialize(e)
    }
}

impl std::fmt::Display for StorageError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Io(e) => write!(f, "Error during I/O: {}", e),
            Self::Serialize(e) => write!(f, "Error serializing data: {}", e),
            Self::Deserialize(e) => write!(f, "Error deserializing data: {}", e),
        }
    }
}

impl std::error::Error for StorageError {}
