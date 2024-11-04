use std::path::PathBuf;
use structopt::StructOpt;

#[derive(Debug, StructOpt)]
#[structopt(
    name = "ricocomputer",
    about = "Drop-in replacement for picoComputer assembler and virtual machine."
)]
pub enum Opt {
    #[structopt(name = "asm", about = "Assemble asm file to binary file for VM")]
    Assemble {
        #[structopt(long, short, parse(from_os_str), help = "")]
        source: PathBuf,

        #[structopt(long, short, parse(from_os_str), help = "")]
        output: Option<PathBuf>,
    },
    #[structopt(name = "run", about = "Run pc file on VM")]
    Run {
        #[structopt(long, short, parse(from_os_str), help = "")]
        source: PathBuf,
    }
}
