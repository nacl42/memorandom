use std::collections::HashMap;

pub struct Value {
    text: String,
    meta: HashMap<String, String>,
}

/*
color {
    
}
 */
impl From<String> for Value {
    fn from(text: String) -> Self {
        Value {
            text,
            meta: Default::default(),
        }
    }
}

impl From<&str> for Value {
    fn from(text: &str) -> Self {
        Value {
            text: text.to_string(),
            meta: Default::default(),
        }
    }
}

impl Value {
    pub fn with<K, V>(mut self, key: K, value: V) -> Self
    where
        K: Into<String>,
        V: Into<String>,
    {
        self.meta.insert(key.into(), value.into());
        self
    }
}


#[cfg(test)]
mod test_value {
    use super::*;

    #[test]
    pub fn test_init() {
        let value = Value::from("apple")
        .with("type", "color");
    }
}