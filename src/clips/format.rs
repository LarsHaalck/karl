use super::clips::{ClipFormatter, Clips};

pub struct TerminalFormatter;

impl ClipFormatter for TerminalFormatter {
    fn print(clips: &Clips, key: Option<char>, unnamed_only: bool) -> Result<(), String> {
        let line_sep = "-------------------------------------------";
        if let Some(key) = key {
            let clip = clips.get(key, unnamed_only)?;
            println!("{}:\t{}", key, clip.0);
        } else {
            clips.named.iter().for_each(|clip| {
                println!("{}:\t{}", clip.0, clip.1 .0);
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

pub struct RawFormatter;

impl ClipFormatter for RawFormatter {
    fn print(clips: &Clips, key: Option<char>, unnamed_only: bool) -> Result<(), String> {
        if let Some(key) = key {
            let clip = clips.get(key, unnamed_only);

            // consume the error in raw formatter
            if let Ok(clip) = clip {
                println!("{},{:?}", key, clip.0);
            }
        } else {
            clips.named.iter().for_each(|clip| {
                println!("{},{:?}", clip.0, clip.1 .0);
            });
            clips.unnamed.iter().enumerate().for_each(|(i, clip)| {
                println!("∅{},{:?}", i, clip.0);
            });
        }
        Ok(())
    }
}

pub struct LineFormatter;

impl ClipFormatter for LineFormatter {
    fn print(clips: &Clips, key: Option<char>, unnamed_only: bool) -> Result<(), String> {
        if let Some(key) = key {
            let clip = clips.get(key, unnamed_only);

            // consume the error in raw formatter
            if let Ok(clip) = clip {
                println!("{}\n{}", key, clip.0);
            }
        } else {
            clips.named.iter().for_each(|clip| {
                println!("{}\n{}", clip.0, clip.1 .0);
            });
            clips.unnamed.iter().enumerate().for_each(|(i, clip)| {
                println!("∅{}\n{}", i, clip.0);
            });
        }
        Ok(())
    }
}
