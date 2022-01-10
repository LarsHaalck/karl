mod clip;
mod clips;
mod format;

use clip::Clip;
use clips::{Clips, ClipKey};
use format::{TerminalFormatter, RofiFormatter};

use std::path::PathBuf;
use structopt::{clap::ArgGroup, StructOpt};
use std::fs;

#[derive(StructOpt, Debug)]
#[structopt(about = "karl: CLI clipboard manager")]
enum KarlArgs {
    Add {
        #[structopt(short, long, help = "Key used for quick access, can be ommited")]
        key: Option<ClipKey>,


        #[structopt(flatten)]
        input: InputArgs,
    },
    Clear {
        #[structopt(short, long, help = "Delete only key", conflicts_with = "unnamed_only")]
        key: Option<ClipKey>,
        #[structopt(
            short,
            long,
            help = "Set if only unnamed entries should be cleared",
            conflicts_with = "key"
        )]
        unnamed_only: bool,
    },
    List {
        #[structopt(flatten)]
        output_type: OutputArgs,

        #[structopt(short, long, help = "List only key")]
        key: Option<ClipKey>,
    },
}

#[derive(StructOpt, Debug)]
#[structopt(group = ArgGroup::with_name("input").required(true))]
struct InputArgs {
    #[structopt(
        short,
        long,
        help = "Read from file",
        group = "input"
    )]
    file: Option<Option<PathBuf>>,

    #[structopt(
        help = "Data of clipboard entry",
        group = "input"
    )]
    value: Option<String>,

    #[structopt(
        short,
        long,
        help = "Get data from clipboard",
        group = "input"
    )]
    clipboard: bool,
}

#[derive(StructOpt, Debug)]
#[structopt(group = ArgGroup::with_name("output_type").required(false))]
struct OutputArgs {
    #[structopt(
        short,
        long,
        help = "Rofi output",
        group = "output_type",
    )]
    rofi: bool,
}

fn handle_args(args: KarlArgs) -> Result<(), String> {
    match args {
        KarlArgs::Add { key, input } => {
            let InputArgs { file, value, clipboard } = input;
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
