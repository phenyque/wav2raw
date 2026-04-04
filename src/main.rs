mod wav2raw;

use crate::wav2raw::Wav2RawError;
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

    let mut infile = match File::open(&args.infile) {
        Ok(file) => file,
        Err(_) => {
            println!("Can't open input file at {}.", args.infile.display());
            return Ok(());
        }
    };

    let mut outfile = match File::create_new(&args.outfile) {
        Ok(file) => file,
        Err(_) => {
            println!("Can't open new output file at {}.", args.outfile.display());
            return Ok(());
        }
    };

    match read_file_header(&mut infile) {
        Err(Wav2RawError::CantReadHeader) => {
            println!(
                "Can't read full RIFF header (first 12 bytes) from {}.",
                args.infile.display()
            );
            return Ok(());
        }
        Err(Wav2RawError::InvalidHeader) => {
            println!(
                "RIFF header of input file {} is invalid.",
                args.infile.display()
            );
            return Ok(());
        }
        _ => {}
    };

    println!(
        "Copying data from {} into {}...",
        args.infile.display(),
        args.outfile.display()
    );
    copy_data(&mut infile, &mut outfile)?;

    Ok(())
}
