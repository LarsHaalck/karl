use crate::clip::Clip;

use directories::ProjectDirs;
use serde::{Deserialize, Serialize};
use serde_json;
use std::collections::HashMap;
use std::fs;
use std::path::PathBuf;

pub type ClipKey = char;

#[derive(Serialize, Deserialize, Debug)]
pub struct Clips {
    pub named: HashMap<ClipKey, Clip>,
    pub unnamed: Vec<Clip>,
}

pub trait ClipFormatter {
    fn print(clips: &Clips, key: Option<ClipKey>) -> Result<(), String>;
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
}