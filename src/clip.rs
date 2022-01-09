use serde::{Deserialize, Serialize};
use std::io::Read;
use std::process::Command;

#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub struct Clip(pub String);

impl Clip {
    pub fn from_file<R: Read>(reader: R) -> Result<Clip, String> {
        let bytes: Result<Vec<_>, _> = reader.bytes().collect();
        let bytes = bytes.map_err(|_| "Error reading bytes from file")?;
        let s: String = std::str::from_utf8(&bytes[..bytes.len() - 1])
            .map_err(|_| "Error converting file content into string")?
            .to_string();
        Ok(Clip::from(s))
    }

    pub fn from_clipboard() -> Result<Clip, String> {
        let output = Command::new("sh")
            .arg("-c")
            .arg("xclip -selection clipboard -o")
            .output()
            .map_err(|_| "Failed to retrieve content from clipboard")?;
        let s = String::from_utf8(output.stdout)
            .map_err(|_| "Error converting clipboard content into string")?;
        Ok(Clip::from(s))
    }
}

impl From<String> for Clip {
    fn from(value: String) -> Clip {
        return Clip(value);
    }
}

