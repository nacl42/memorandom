use std::{fmt::{Display, Formatter}};

use multimap::MultiMap;

pub type Key = String;
pub type Value = String;

// TODO: insert_vec (DONE)
// TODO: count all elements
// TODO: attributes

#[derive(Debug, Clone)]
pub struct Memo {
    collection: Key,            // a schema
    title: Value,               // an id
    data: MultiMap<Key, Value>, // the actual information
}


impl Memo {
    pub fn new<K, V>(collection: K, title: V) -> Self
    where
        K: Into<Key>,
        V: Into<Value>,
    {
        Self {
            collection: collection.into(),
            title: title.into(),
            data: Default::default(),
        }
    }

    pub fn insert<K, V>(&mut self, key: K, value: V)
    where
        K: Into<Key>,
        V: Into<Value>,
    {
        self.data.insert(key.into(), value.into()); // TODO: TBR
    }

    pub fn insert_many<K, V, I>(&mut self, key: K, values: I)
    where
        K: Into<Key>,
        V: Into<Value>,
        I: IntoIterator<Item = V>,
    {
        self.data
            .insert_many(key.into(), values.into_iter().map(V::into));       
    }

    pub fn with<K, V>(mut self, key: K, value: V) -> Self
    where
        K: Into<Key>,
        V: Into<Value>,
    {
        self.insert(key, value);
        self
    }

    pub fn with_many<K, V, I>(mut self, key: K, values: I) -> Self
    where
        K: Into<Key>,
        V: Into<Value>,
        I: IntoIterator<Item = V>,
    {
        self.insert_many(key, values);
        self
    }
}

impl Display for Memo {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), std::fmt::Error> {
        writeln!(f, "@{} {}", self.collection, self.title)?;
        for (key, values) in self.data.iter_all() {
           for value in values {
            writeln!(f, ".{} {}", key, value)?;
           }
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn new_memo() {
        let memo = Memo::new("book", "The Lord of the Rings")
            .with("author", "J.R.R. Tolkien")
            .with("character", "Bilbo Baggins")
            .with("character", "Samwise Gamgee")
            .with("character", "Gandalf the Gray");

        assert_eq!(memo.collection, "book".to_string());
        assert_eq!(memo.title, "The Lord of the Rings".to_string());
        assert_eq!(memo.data.len(), 2);
        //assert_eq!(memo.data.total_len(), 4); // DOES NOT EXIST IN MULTIMAP
        assert_eq!(
            memo.data.get_vec("author").map(|values| values.len()),
            Some(1)
        );
        assert_eq!(
            memo.data.get_vec("character").map(|values| values.len()),
            Some(3)
        );
    }

    #[test]
    fn new_memo_with() {
        let memo = Memo::new("book", "The Lord of the Rings").with_many(
            "character",
            ["Bilbo Baggins", "Samwise Gamgee", "Gandalf the Gray"],
        );
        assert_eq!(
            memo.data.get_vec("character").map(|values| values.len()),
            Some(3)
        );
    }
}
