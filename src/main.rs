use clap::Parser;
use std::fs::File;
use std::io::Read;
use std::path::PathBuf;

#[derive(Parser)]
#[clap()]
struct Args {
    infile: PathBuf,
    outfile: PathBuf,
}

struct RiffWaveHeader {
    riff_signature: u32,
    filesize: u32,
    wave_signature: u32,
}

#[derive(Debug)]
enum Wave2RawError {
    CantReadHeader,
    InvalidHeader,
}

impl std::fmt::Display for Wave2RawError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match &self {
            Wave2RawError::CantReadHeader => write!(f, "Can't read header from input file"),
            Wave2RawError::InvalidHeader => write!(f, "Invalid header found in file"),
        }
    }
}

impl std::error::Error for Wave2RawError {}

fn read_header(infile: &mut File) -> RiffWaveHeader {
    let mut buffer: [u8; 4] = [0; 4];

    let riff_signature = match infile.read_exact(&mut buffer) {
        Ok(()) => u32::from_le_bytes(buffer),
        Err(_) => 0,
    };
    let filesize = match infile.read_exact(&mut buffer) {
        Ok(()) => u32::from_le_bytes(buffer),
        Err(_) => 0,
    };
    let wave_signature = match infile.read_exact(&mut buffer) {
        Ok(()) => u32::from_le_bytes(buffer),
        Err(_) => 0,
    };

    RiffWaveHeader {
        riff_signature,
        filesize,
        wave_signature,
    }
}

fn validate_header_and_get_filesize(file_header: RiffWaveHeader) -> Result<u32, Wave2RawError> {
    match file_header {
        RiffWaveHeader {
            riff_signature: 1179011410,
            filesize: file_size,
            wave_signature: 1163280727,
        } => Ok(file_size),
        RiffWaveHeader {
            riff_signature: 0,
            filesize: 0,
            wave_signature: 0,
        } => Err(Wave2RawError::CantReadHeader),
        _ => {
            println!(
                "riff: {}, filesize: {}, wave: {}",
                file_header.riff_signature, file_header.filesize, file_header.wave_signature
            );
            Err(Wave2RawError::InvalidHeader)
        }
    }
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = Args::parse();

    println!("{} {}", args.infile.display(), args.outfile.display());

    let mut infile = File::open(args.infile)?;
    let fileheader = read_header(&mut infile);

    validate_header_and_get_filesize(fileheader)?;

    Ok(())
}
