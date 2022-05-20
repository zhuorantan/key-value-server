use serde_json::{Result, Map, Value};

type Object = Map<String, Value>;

#[derive(Debug)]
pub struct Storage {
    data: Object,
}

impl Storage {
    pub fn from_file() -> Result<Storage> {
        Ok(Storage { data: Map::new() })
    }
}

impl Storage {
    pub fn update(&mut self, path: String, payload: String) {
        let value: Value = serde_json::from_str(&payload).unwrap_or(Value::String(payload));

        let mut path_parts = self.parse_path(&path);
        let last = path_parts.pop().unwrap();
        let target = self.get_or_insert(&path_parts);
        target.insert(last, value);
    }

    fn parse_path(&self, path: &str) -> Vec<String> {
        path.split('/').map(|s| s.to_string()).collect::<Vec<String>>()
    }

    fn get_or_insert(&mut self, path_parts: &Vec<String>) -> &mut Object {
        let mut current = &mut self.data;
        for part in path_parts {
            if !current.contains_key(part) {
                current.insert(part.to_string(), Value::Object(Map::new()));
            }
            current = current
                .get_mut(part)
                .unwrap()
                .as_object_mut()
                .expect("Expected object");
        }
        current
    }
}
