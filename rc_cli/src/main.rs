mod opt;
mod commands;

use structopt::StructOpt;
use miette::Result;

use opt::Opt;
use commands::{asm, run};

fn main() -> Result<()> {
    let opts: Opt = Opt::from_args();

    match opts {
        Opt::Assemble { source, output } => asm(&source, output.as_ref()),
        Opt::Run { source } => run(&source),
    }
}
