use clap::Parser;
use std::path::PathBuf;

#[derive(Parser)]
#[clap()]
struct Args {
    infile: PathBuf,
    outfile: PathBuf,
}

fn main() {
    let args = Args::parse();

    println!("{} {}", args.infile.display(), args.outfile.display());
}
