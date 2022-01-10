mod cli;
mod clips;

use cli::{InputArgs, KarlArgs, OutputArgs};
use clips::{
    format::{RofiFormatter, TerminalFormatter},
    Clip, Clips,
};
use structopt::StructOpt;

use std::fs;

fn handle_args(args: KarlArgs) -> Result<(), String> {
    match args {
        KarlArgs::Add { key, input } => {
            let InputArgs {
                file,
                value,
                clipboard,
            } = input;
            let mut clips = Clips::read();
            if let Some(value) = value {
                clips.add(key, Clip::from(value));
            } else if let Some(file) = file {
                let clip = match file {
                    Some(f) => Clip::from_file(fs::File::open(f).map_err(|_| "Fuck")?),
                    None => Clip::from_file(std::io::stdin()),
                }?;
                clips.add(key, clip);
            } else if clipboard {
                clips.add(key, Clip::from_clipboard()?);
            }
            clips.write()?;
        }
        KarlArgs::Clear { key, unnamed_only } => {
            let mut clips = Clips::read();
            clips.clear(key, unnamed_only)?;
            clips.write()?;
        }
        KarlArgs::List { key, output_type } => {
            let clips = Clips::read();
            let OutputArgs { rofi } = output_type;
            if rofi {
                clips.print(key, RofiFormatter)?;
            } else {
                clips.print(key, TerminalFormatter)?;
            }
        }
    }
    Ok(())
}

fn main() {
    let args = KarlArgs::from_args();
    if let Err(e) = handle_args(args) {
        eprintln!("Error: {}", e);
    }
}
