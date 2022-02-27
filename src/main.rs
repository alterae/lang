use clap::StructOpt;

mod cli;

fn main() {
    let opts = cli::Opts::parse();

    println!("{opts:?}");
}
