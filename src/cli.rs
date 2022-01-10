use std::path::PathBuf;
use structopt::{clap::ArgGroup, StructOpt};

#[derive(StructOpt, Debug)]
#[structopt(about = "karl: CLI clipboard manager")]
pub enum KarlArgs {
    Add {
        #[structopt(short, long, help = "Key used for quick access, can be ommited")]
        key: Option<char>,


        #[structopt(flatten)]
        input: InputArgs,
    },
    Clear {
        #[structopt(short, long, help = "Delete only key")]
        key: Option<char>,
        #[structopt(
            short,
            long,
            help = "Set if only unnamed entries should be cleared",
        )]
        unnamed_only: bool,
    },
    List {
        #[structopt(flatten)]
        output_type: OutputArgs,

        #[structopt(short, long, help = "List only key")]
        key: Option<char>,
    },
}

#[derive(StructOpt, Debug)]
#[structopt(group = ArgGroup::with_name("input").required(true))]
pub struct InputArgs {
    #[structopt(
        short,
        long,
        help = "Read from file",
        group = "input"
    )]
    pub file: Option<Option<PathBuf>>,

    #[structopt(
        help = "Data of clipboard entry",
        group = "input"
    )]
    pub value: Option<String>,

    #[structopt(
        short,
        long,
        help = "Get data from clipboard",
        group = "input"
    )]
    pub clipboard: bool,
}

#[derive(StructOpt, Debug)]
#[structopt(group = ArgGroup::with_name("output_type").required(false))]
pub struct OutputArgs {
    #[structopt(
        short,
        long,
        help = "Rofi output",
        group = "output_type",
    )]
    pub rofi: bool,
}
