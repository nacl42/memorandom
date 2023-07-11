use std::fmt::{Display, Formatter};
use serde_derive::{Serialize};

use multimap::MultiMap;

pub type Value = String;


// TODO: insert_vec (DONE)
// TODO: count all elements
// TODO: attributes

// TODO: Value could be value with attributes


#[derive(Debug, Clone, PartialEq, Serialize)]
pub struct Memo<'a> {
    data: MultiMap<&'a str, Value>, // the actual information
}

impl<'a> Memo<'a> {
    pub fn new<K, V>(collection: K, title: V) -> Self
    where
        K: Into<&'a str>,
        V: Into<Value>,
    {
        let mut data: MultiMap<&'a str, Value> = Default::default();
        data.insert("mr:id", title.into());
        data.insert("mr:schema", Value::from(collection.into()));
        Self {
            data,
        }
    }

    pub fn title(&'a self) -> &'a str {
        self.data.get("mr:id").unwrap()
    }

    pub fn collection(&'a self) -> &'a str {
        self.data.get("mr:schema").unwrap()
    }

    pub fn data(&self) -> &MultiMap<&'a str, Value> {
        &self.data
    }

    pub fn insert<K, V>(&mut self, key: K, value: V)
    where
        K: Into<&'a str>,
        V: Into<Value>,
    {
        let key = key.into();
        let value = value.into();
        self.data.insert(key.into(), value.clone()); // TODO: TBR
    }

    pub fn insert_many<K, V, I>(&mut self, key: K, values: I)
    where
        K: Into<&'a str>,
        V: Into<Value>,
        I: IntoIterator<Item = V>,
    {
        let key = key.into();
        let values = values.into_iter();
        
        self.data  // TODO: TBR
            .insert_many(key.into(), values.map(V::into));

    }

    pub fn with<K, V>(mut self, key: K, value: V) -> Self
    where
        K: Into<&'a str>,
        V: Into<Value>,
    {
        self.insert(key, value);
        self
    }

    pub fn with_many<K, V, I>(mut self, key: K, values: I) -> Self
    where
        K: Into<&'a str>,
        V: Into<Value>,
        I: IntoIterator<Item = V>,
    {
        self.insert_many(key, values);
        self
    }
}

impl<'a> Display for Memo<'a> {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), std::fmt::Error> {
        writeln!(f, "@{} {}", self.collection(), self.title())?;
        for (key, values) in self.data.iter_all() {
            for value in values {
                writeln!(f, ".{} {}", key, value)?;
            }
        }
        Ok(())
    }
}

#[cfg(test)]
mod test_memo {
    use super::*;

    #[test]
    fn new_memo() {
        let memo = Memo::new("book", "The Lord of the Rings")
            .with("author", "J.R.R. Tolkien")
            .with("character", "Bilbo Baggins")
            .with("character", "Samwise Gamgee")
            .with("character", "Gandalf the Gray");

        assert_eq!(memo.collection(), "book");
        assert_eq!(memo.title(), "The Lord of the Rings");
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

