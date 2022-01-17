mod cli;
mod clips;

use cli::{InputArgs, KarlArgs, OutputArgs};
use clips::{
    format::{TerminalFormatter, RawFormatter, LineFormatter},
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
        KarlArgs::List { key, unnamed_only, output_type } => {
            let clips = Clips::read();
            let OutputArgs { raw, line } = output_type;
            if raw {
                clips.print(key, unnamed_only, RawFormatter)?;
            } else if line {
                clips.print(key, unnamed_only, LineFormatter)?;
            } else {
                clips.print(key, unnamed_only, TerminalFormatter)?;
            }
        }
    }
    Ok(())
}

fn main() -> Result<(), String> {
    let args = KarlArgs::from_args();
    handle_args(args)
}
