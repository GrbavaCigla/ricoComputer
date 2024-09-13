use structopt::StructOpt;

mod opt;
mod commands;
mod parsers;
mod types;

use opt::Opt;
use commands::{compile, run};

fn main() {
    let opts: Opt = Opt::from_args();

    match opts {
        Opt::Compile { source, output } => {
            compile(&source, output.as_ref());
        },
        Opt::Run { source } => {
            run(&source);
        }
    }
}
