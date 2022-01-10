use crate::clips::{Clips, ClipFormatter, ClipKey};

pub struct TerminalFormatter;

impl ClipFormatter for TerminalFormatter {
    fn print(clips: &Clips, key: Option<ClipKey>) -> Result<(), String> {
        let line_sep = "-------------------------------------------";
        if let Some(key) = key {
            let clip = clips
                .named
                .get(&key)
                .ok_or(format!("Key {} does not exist", key))?;
            println!("{}:\t{}", key, clip.0);
            println!("{}", line_sep);
        } else {
            clips.named.iter().for_each(|clip| {
                println!("{}:\t{}", clip.0, clip.1.0);
                println!("{}", line_sep);
            });
            clips.unnamed.iter().enumerate().for_each(|(i, clip)| {
                println!("∅{}:\t{}", i, clip.0);
                println!("{}", line_sep);
            });
        }
        Ok(())
    }
}

pub struct RofiFormatter;

impl ClipFormatter for RofiFormatter {
    fn print(clips: &Clips, key: Option<ClipKey>) -> Result<(), String> {
        if let Some(key) = key {
            let clip = clips
                .named
                .get(&key)
                .ok_or(format!("Key {} does not exist", key))?;
            println!("{},{:?}", key, clip.0);
        } else {
            clips.named.iter().for_each(|clip| {
                println!("{},{:?}", clip.0, clip.1.0);
            });
            clips.unnamed.iter().enumerate().for_each(|(i, clip)| {
                println!("∅{},{:?}", i, clip.0);
            });
        }
        Ok(())
    }
}
