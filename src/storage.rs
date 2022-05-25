use std::path;
use std::fs;
use serde_json::{Map, Value};

type Object = Map<String, Value>;

#[derive(Debug)]
pub enum Error {
  NotAnObject(String),
  NoKey,
}

#[derive(Debug)]
pub struct Storage {
    data: Value,
    path: Option<String>,
}

impl Storage {
    pub fn from_file(path: Option<String>) -> Storage {
        let object: Object;
        if let Some(ref path) = path {
            if path::Path::new(&path).exists() {
                let content = fs::read_to_string(&path)
                    .expect("Failed to read file");
                object = serde_json::from_str(&content)
                    .expect("Failed to parse JSON");
            } else {
                object = Object::new();
            }
        } else {
            object = Object::new();
        }

        Storage { data: Value::Object(object), path }
    }
}

impl Storage {
    pub fn get(&self, path: &str) -> Option<&Value> {
        let mut current = &self.data;
        for part in self.parse_path(path) {
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

    pub fn update(&mut self, path: &str, payload: &str) -> Result<(), Error> {
        let mut path_parts = self.parse_path(path);

        match path_parts.pop() {
            Some(last) => {
                match self.get_object_or_insert(&path_parts) {
                    Ok(target) => {
                        let value: Value = serde_json::from_str(&payload).unwrap_or(Value::String(payload.to_string()));
                        target.insert(last, value);
                        self.save();
                        Ok(())
                    },
                    Err(error) => Err(error),
                }
            },
            None => Err(Error::NoKey),
        }
    }

    pub fn delete(&mut self, path: &str) -> Result<(), Error> {
        let mut path_parts = self.parse_path(path);

        match path_parts.pop() {
            Some(last) => {
                match self.get_object_or_insert(&path_parts) {
                    Ok(target) => {
                        target.remove(&last);
                        self.save();
                        Ok(())
                    },
                    Err(error) => Err(error),
                }
            },
            None => Err(Error::NoKey),
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

    fn save(&self) {
        match self.path {
            Some(ref path) => {
                let directory = path::Path::new(&path).parent().expect("Failed to get parent directory");
                if !directory.exists() {
                    fs::create_dir_all(directory).expect("Failed to create directory");
                }

                let content = serde_json::to_string(&self.data).expect("Failed to serialize JSON");
                fs::write(path, content).expect("Failed to write file");
            },
            None => (),
        }
    }
}
