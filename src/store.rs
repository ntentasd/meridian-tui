use std::collections::HashMap;

use crate::api::{
    Event,
    Kind::{Delete, Upsert},
    Route,
};

pub struct Store {
    map: HashMap<(String, String), Route>,
}

impl Store {
    pub fn new() -> Store {
        Store {
            map: HashMap::new(),
        }
    }

    pub fn apply(&mut self, event: Event) {
        let key = (event.entry.uid.clone(), event.entry.hostname.clone());

        match event.kind {
            Upsert => self.map.insert(key, event.entry),
            Delete => self.map.remove(&key),
        };
    }

    pub fn clear(&mut self) {
        self.map.clear();
    }

    pub fn routes(&self) -> Vec<&Route> {
        let mut routes = self.map.values().collect::<Vec<_>>();
        routes.sort_by(|a, b| {
            (&a.namespace, &a.name, &a.hostname).cmp(&(&b.namespace, &b.name, &b.hostname))
        });

        routes
    }
}
