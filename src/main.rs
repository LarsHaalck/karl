mod clip;
use clip::{Clip, ClipKey, Clips};
use std::path::PathBuf;
use structopt::StructOpt;
use std::fs;

#[derive(StructOpt, Debug)]
#[structopt(about = "karl: CLI clipboard manager")]
enum KarlArgs {
    Add {
        #[structopt(short, long, help = "Key used for quick access, can be ommited")]
        key: Option<ClipKey>,

        #[structopt(
            short,
            long,
            help = "Read from file",
            conflicts_with = "value",
            required_unless = "value"
        )]
        file: Option<Option<PathBuf>>,

        #[structopt(
            help = "Data of clipboard entry",
            conflicts_with = "file",
            required_unless = "file"
        )]
        value: Option<String>,
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
        #[structopt(short, long, help = "List only key")]
        key: Option<ClipKey>,
    },
}


fn handle_args(args: KarlArgs) -> Result<(), String> {
    match args {
        KarlArgs::Add { key, file, value } => {
            let mut clips = Clips::read();
            if let Some(value) = value {
                clips.add(key, Clip::from(value));
            } else if let Some(file) = file {
                let clip = match file {
                    Some(f) => Clip::from_file(fs::File::open(f).map_err(|_| "Fuck")?),
                    None => Clip::from_file(std::io::stdin()),
                }?;
                clips.add(key, clip);
            }
            clips.write()?;

        }
        KarlArgs::Clear { key, unnamed_only } => {
            let mut clips = Clips::read();
            clips.clear(key, unnamed_only)?;
            clips.write()?;
        }
        KarlArgs::List { key } => {
            let clips = Clips::read();
            clips.print(key)?;
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
