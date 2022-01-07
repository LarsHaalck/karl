mod clip;
use clip::Clip;

use structopt::StructOpt;

#[derive(StructOpt, Debug)]
#[structopt(about = "karl: CLI clipboard manager")]
enum Karl {
    Add {
        #[structopt(short, long, help = "Key used for quick access, can be ommited")]
        key: Option<String>,
        #[structopt(help = "Actual data of clipboard entry")]
        data: String,
    },
    Clear {
        #[structopt(short, long, help = "Set if only unnamed entries should be cleared")]
        unnamed_only: bool,
    },
    List,
}

fn main() {
    let args = Karl::from_args();
    println!("{:?}", args);
}
