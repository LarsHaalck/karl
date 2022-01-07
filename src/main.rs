mod clip;
use clip::{Clip, Clips};
use structopt::StructOpt;

#[derive(StructOpt, Debug)]
#[structopt(about = "karl: CLI clipboard manager")]
enum Karl {
    Add {
        #[structopt(short, long, help = "Key used for quick access, can be ommited")]
        key: Option<String>,
        #[structopt(help = "Actual data of clipboard entry")]
        value: String,
    },
    Clear {
        #[structopt(short, long, help = "Set if only unnamed entries should be cleared")]
        unnamed_only: bool,
    },
    List,
}

fn main() {
    let args = Karl::from_args();
    match args {
        Karl::Add { key, value } => {
            let clips = Clips::read();
            clips.add(key, Clip(value)).unwrap();
        }
        Karl::Clear { unnamed_only } => {}
        Karl::List => {
            let clips = Clips::read();
            clips.named.iter().for_each(|clip| {
                println!("{:?}, {:?}", clip.0, clip.1);
            });
            clips.unnamed.iter().for_each(|clip| {
                println!("{:?}", clip);
            });
        }
    }
}
