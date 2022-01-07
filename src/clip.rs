use directories::ProjectDirs;
use serde::{Deserialize, Serialize};
use serde_json;
use std::collections::HashMap;
use std::fs;
use std::path::{Path, PathBuf};

#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub struct Clip(pub String);

#[derive(Serialize, Deserialize, Debug)]
pub struct Clips {
    pub named: HashMap<String, Clip>,
    pub unnamed: Vec<Clip>,
}

impl Clips {
    fn get_dir() -> PathBuf {
        let dir = ProjectDirs::from("karl", "karl", "karl").unwrap();
        dir.cache_dir().join("clips.data")
    }

    pub fn new() -> Clips {
        Clips {
            named: HashMap::new(),
            unnamed: Vec::new(),
        }
    }

    pub fn read() -> Clips {
        let cache = Clips::get_dir();
        if cache.exists() {
            let bin = fs::read_to_string(cache).ok().unwrap_or(String::new());
            let deserialized: Clips = serde_json::from_str(&bin).ok().unwrap_or(Clips::new());
            deserialized
        } else {
            Clips::new()
        }
    }

    fn write(&self) -> Result<(), String> {
        let cache = Clips::get_dir();
        if !cache.parent().unwrap().exists() {
            fs::create_dir(cache.parent().unwrap())
                .or_else(|_| Err("Could not create cache directory"))?;
        }
        let res = serde_json::to_string(self).or_else(|_| Err("Could not serialize Clips"))?;
        fs::write(cache, res).or_else(|_| Err("Could not write to cache file"))?;
        Ok(())
    }

    pub fn add(mut self, key: Option<String>, clip: Clip) -> Result<(), String> {
        if let Some(key) = key {
            self.named.insert(key, clip);
        } else {
            if !self.unnamed.contains(&clip) {
                self.unnamed.push(clip);
            }
        }
        self.write()
    }
}
