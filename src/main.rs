mod wav2raw;

use crate::wav2raw::copy_data;
use crate::wav2raw::read_file_header;
use clap::Parser;
use std::fs::File;
use std::path::PathBuf;

#[derive(Parser)]
#[clap()]
struct Args {
    infile: PathBuf,
    outfile: PathBuf,
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = Args::parse();

    println!("{} {}", args.infile.display(), args.outfile.display());

    let mut infile = File::open(args.infile)?;
    let mut outfile = File::create_new(args.outfile)?;

    read_file_header(&mut infile)?;

    copy_data(&mut infile, &mut outfile)?;

    Ok(())
}
