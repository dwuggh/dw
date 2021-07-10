#![allow(unused)]

use std::collections::HashMap;
use patricia_tree::PatriciaMap;

/// The container for dictionary, can choose backend.
#[derive(Debug, Clone)]
pub enum Map<V> {
    PatriciaTree(PatriciaMap<V>),
    HashMap(HashMap<String, V>),
}

impl<V> Map<V> {
    pub fn new_ptree() -> Map<V> {
        Map::PatriciaTree(PatriciaMap::new())
    }
    pub fn new_hashmap() -> Map<V> {
        Map::HashMap(HashMap::new())
    }
}


impl<V> Map<V> {
    pub fn insert<S: Into<String>>(&mut self, key: S, val: V) -> Option<V> {
        match self {
            Map::PatriciaTree(m) => {
                m.insert(key.into(), val)
            }
            Map::HashMap(m) => {
                m.insert(key.into(), val)
            }
        }
    }

    pub fn get(&self, key: &str) -> Option<&V> {
        match self {
            Map::PatriciaTree(m) => {
                m.get(key)
            }
            Map::HashMap(m) => {
                m.get(key)
            }
        }
    }

    pub fn get_mut(&mut self, key: &str) -> Option<&mut V> {
        match self {
            Map::PatriciaTree(m) => {
                m.get_mut(key)
            }
            Map::HashMap(m) => {
                m.get_mut(key)
            }
        }
    }
}
