//! A Node represents a single piece of information, a Key-Value pair.
//!
//! A Node consists of key, a value and a map of attributes.
//!

pub type Key = String;
pub type Value = String;

use std::collections::HashMap;

/// A Node consists of key, a value and a map of attributes.
#[derive(Debug, PartialEq)]
pub struct Node {
    pub key: Key,
    pub value: Value,
    pub attrs: HashMap<Key, Value>
}

impl Node {
    pub fn new<K, V>(key: K, value: V) -> Self
    where K: Into<Key>, V: Into<Value>
    {
        Node {
            key: key.into(),
            value: value.into(),
            attrs: HashMap::new()
        }
    }

    pub fn with<K, V>(mut self, key: K, value: V) -> Self
    where K: Into<Key>, V: Into<Value>
    {
        self.attrs.insert(key.into(), value.into());
        self
    }

    pub fn with_many(mut self, attrs: HashMap<Key, Value>) -> Self
    {
        self.attrs = attrs;
        self
    }

    pub fn attrs<'a>(&'a self) -> &'a HashMap<Key, Value>
    {
        &self.attrs
    }
}

