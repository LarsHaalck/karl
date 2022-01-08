use directories::ProjectDirs;
use serde::{Deserialize, Serialize};
use serde_json;
use std::collections::HashMap;
use std::fs;
use std::io::Read;
use std::path::PathBuf;

pub type ClipKey = char;

#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub struct Clip(pub String);

#[derive(Serialize, Deserialize, Debug)]
pub struct Clips {
    pub named: HashMap<ClipKey, Clip>,
    pub unnamed: Vec<Clip>,
}

impl Clip {
    pub fn from_file<R: Read>(reader: R) -> Result<Clip, String> {
        let bytes: Result<Vec<_>, _> = reader.bytes().collect();
        let bytes = bytes.map_err(|_| "Error reading bytes from file")?;
        let s: String = std::str::from_utf8(&bytes[..bytes.len() - 1])
            .map_err(|_| "Error converting file content into string")?
            .to_string();
        Ok(Clip::from(s))
    }
}

impl From<String> for Clip {
    fn from(value: String) -> Clip {
        return Clip(value);
    }
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

    pub fn write(&self) -> Result<(), String> {
        let cache = Clips::get_dir();
        if !cache.parent().unwrap().exists() {
            fs::create_dir(cache.parent().unwrap())
                .map_err(|_| "Could not create cache directory")?;
        }
        let res = serde_json::to_string(self).map_err(|_| "Could not serialize Clips")?;
        fs::write(cache, res).map_err(|_| "Could not write to cache file")?;
        Ok(())
    }

    pub fn add(&mut self, key: Option<ClipKey>, clip: Clip) {
        if let Some(key) = key {
            self.named.insert(key, clip);
        } else {
            if !self.unnamed.contains(&clip) {
                self.unnamed.push(clip);
            }
        }
    }

    pub fn clear(&mut self, key: Option<ClipKey>, unnamed_only: bool) -> Result<(), String> {
        if let Some(key) = key {
            self.named
                .remove(&key)
                .ok_or(format!("Key {} did not exist.", key))?;
        } else if unnamed_only {
            self.unnamed.clear();
        } else {
            self.named.clear();
            self.unnamed.clear();
        }
        Ok(())
    }

    pub fn print(&self, key: Option<ClipKey>) -> Result<(), String> {
        if let Some(key) = key {
            let clip = self
                .named
                .get(&key)
                .ok_or(format!("Key {} does not exist", key))?;
            println!("{}:\t{}", key, clip.0);
        } else {
            self.named.iter().for_each(|clip| {
                println!("{}:\t{}", clip.0, clip.1.0);
            });
            self.unnamed.iter().for_each(|clip| {
                println!("\t{}", clip.0);
            });
        }
        Ok(())
    }
}
