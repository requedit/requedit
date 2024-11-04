use std::collections::HashMap;

use crate::{error::RequeditError, proxy::data::ProxyData};

pub(crate) struct ProxyDataStore {
    store: HashMap<String, ProxyData>,
}

impl ProxyDataStore {
    pub(crate) fn new() -> Self {
        Self {
            store: HashMap::new(),
        }
    }

    pub(crate) fn insert_or_update(&mut self, data: ProxyData) -> Result<ProxyData, RequeditError> {
        if self.store.contains_key(&data.id) {
            // update
            self.update(data)
        } else {
            // insert
            self.insert(data)
        }
    }

    fn insert(&mut self, data: ProxyData) -> Result<ProxyData, RequeditError> {
        self.store.insert(data.id.clone(), data.clone());
        Ok(data)
    }
    fn update(&mut self, data: ProxyData) -> Result<ProxyData, RequeditError> {
        if let Some(existing_data) = self.store.remove(&data.id) {
            let updated_data = ProxyData {
                res: data.res.clone(),
                ..existing_data
            };
            Ok(updated_data)
        } else {
            Err(RequeditError::Other("Proxy data not found".to_string()))
        }
    }

    // fn get(&self, key: &str) -> Option<&ProxyData> {
    //     self.store.get(key)
    // }
    // fn delete( &mut self, key: &str) -> Option<ProxyData> {
    //     self.store.remove(key)
    // }
}
