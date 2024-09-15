use structopt::StructOpt;

mod commands;
mod opt;
mod parsers;
mod types;

use commands::{compile, run};
use opt::Opt;

use miette::Result;

fn main() -> Result<()> {
    let opts: Opt = Opt::from_args();

    match opts {
        Opt::Compile { source, output } => compile(&source, output.as_ref()),
        Opt::Run { source } => run(&source),
    }
}
