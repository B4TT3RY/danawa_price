use std::{collections::HashMap, path::Path};

use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Default, Copy, Clone)]
pub struct PriceData {
    pub card_price: Option<i32>,
    pub cash_price: Option<i32>,
}

#[derive(Serialize, Deserialize, Default)]
pub struct PriceStorage {
    price_map: HashMap<String, PriceData>
}

impl PriceStorage {
    pub fn load(path: impl AsRef<Path>) -> Option<Self> {
        let data = std::fs::read(path).ok()?;
        let storage = toml::from_slice(&data).ok()?;
        Some(Self {
            price_map: storage
        })
    }

    pub fn save(&self, path: impl AsRef<Path>) {
        let data = toml::to_vec(&self.price_map).unwrap();
        std::fs::write(path, data).unwrap();
    }

    pub fn get(&self, code: &str) -> PriceData {
        self.price_map.get(code).copied().unwrap_or_else(Default::default)
    }

    pub fn insert(&mut self, code: String, new_data: PriceData) {
        self.price_map.insert(code, new_data);
    }
}