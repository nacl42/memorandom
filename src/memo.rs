use multimap::MultiMap;

pub type Key = String;
pub type Value = String;

// TODO: insert_vec
// TODO: count all elements
// TODO: attributes

#[derive(Debug)]
pub struct Memo {
    collection: Key,
    title: Value,
    data: MultiMap<Key, Value>,
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
        self.data.insert(key.into(), value.into());
    }

    pub fn insert_many<K, V, I>(&mut self, key: K, values: I) 
    where
    K: Into<Key>,
    V: Into<Value>,
    I: IntoIterator<Item=V>
    {
        self.data.insert_many(key.into(), values.into_iter().map(V::into));
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
