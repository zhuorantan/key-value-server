use serde_json::{Map, Value};

type Object = Map<String, Value>;

#[derive(Debug)]
pub enum Error {
  NotAnObject(String),
}

#[derive(Debug)]
pub struct Storage {
    data: Value,
}

impl Storage {
    pub fn from_file() -> Result<Storage, Error> {
        Ok(Storage { data: Value::Object(Map::new()) })
    }
}

impl Storage {
    pub fn get(&self, path: String) -> Option<&Value> {
        let mut current = &self.data;
        for part in self.parse_path(&path) {
            if part.is_empty() {
                continue;
            }
            match current.get(part) {
                Some(value) => current = value,
                None => return None,
            }
        }
        Some(current)
    }

    pub fn update(&mut self, path: String, payload: String) -> Result<(), Error> {
        let value: Value = serde_json::from_str(&payload).unwrap_or(Value::String(payload));

        let mut path_parts = self.parse_path(&path);
        let last = path_parts.pop().unwrap();

        match self.get_object_or_insert(&path_parts) {
            Ok(target) => {
                target.insert(last, value);
                Ok(())
            }
            Err(e) => Err(e),
        }
    }

    pub fn delete(&mut self, path: String) -> Result<(), Error> {
        let mut path_parts = self.parse_path(&path);
        let last = path_parts.pop().unwrap();

        match self.get_object_or_insert(&path_parts) {
            Ok(target) => {
                target.remove(&last);
                Ok(())
            }
            Err(e) => Err(e),
        }
    }

    fn parse_path(&self, path: &str) -> Vec<String> {
        path
            .split('/')
            .filter(|part| !part.is_empty())
            .map(|s| s.to_string())
            .collect::<Vec<String>>()
    }

    fn get_object_or_insert(&mut self, path_parts: &Vec<String>) -> Result<&mut Object, Error> {
        let mut current = self.data.as_object_mut().unwrap();
        for part in path_parts {
            if !current.contains_key(part) {
                current.insert(part.to_string(), Value::Object(Map::new()));
            }

            let next = current
                .get_mut(part)
                .unwrap()
                .as_object_mut();

            match next {
                Some(next) => current = next,
                None => return Err(Error::NotAnObject(part.to_string())),
            }
        }
        Ok(current)
    }
}
