use super::clip::Clip;

use directories::ProjectDirs;
use serde::{Deserialize, Serialize};
use serde_json;
use std::collections::BTreeMap;
use std::fs;
use std::path::PathBuf;

#[derive(Serialize, Deserialize, Debug)]
pub struct Clips {
    pub named: BTreeMap<char, Clip>,
    pub unnamed: Vec<Clip>,
}

pub trait ClipFormatter {
    fn print(clips: &Clips, key: Option<char>, unnamed_only: bool) -> Result<(), String>;
}

impl Clips {
    fn get_dir() -> PathBuf {
        let dir = ProjectDirs::from("karl", "karl", "karl").unwrap();
        dir.cache_dir().join("clips.data")
    }

    pub fn new() -> Clips {
        Clips {
            named: BTreeMap::new(),
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

    pub fn add(&mut self, key: Option<char>, clip: Clip) {
        if let Some(key) = key {
            self.named.insert(key, clip);
        } else {
            if !self.unnamed.contains(&clip) {
                self.unnamed.push(clip);
            }
        }
    }

    pub fn get(&self, key: char, unnamed_only: bool) -> Result<&Clip, String> {
        if unnamed_only {
            let key = key
                .to_string()
                .parse::<usize>()
                .map_err(|_| "Could not convert key into number")?;
            Ok(self
                .unnamed
                .get(key)
                .ok_or(format!("Key {} does not exist", key))?)
        } else {
            Ok(self
                .named
                .get(&key)
                .ok_or(format!("Key {} does not exist", key))?)
        }
    }

    pub fn clear(&mut self, key: Option<char>, unnamed_only: bool) -> Result<(), String> {
        if let Some(key) = key {
            if unnamed_only {
                let num = key
                    .to_string()
                    .parse::<usize>()
                    .map_err(|_| "Could not convert into number")?;
                if num < self.unnamed.len() {
                    self.unnamed.swap_remove(num);
                } else {
                    return Err("Number exceeds number of entries".to_string());
                }
            } else {
                self.named
                    .remove(&key)
                    .ok_or(format!("Key {} did not exist.", key))?;
            }
        } else if unnamed_only {
            self.unnamed.clear();
        } else {
            self.named.clear();
            self.unnamed.clear();
        }
        Ok(())
    }

    pub fn print<T: ClipFormatter>(
        &self,
        key: Option<char>,
        unnamed_only: bool,
        _: T,
    ) -> Result<(), String> {
        T::print(self, key, unnamed_only)
    }
}
