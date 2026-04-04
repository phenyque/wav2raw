use clap::Parser;
use std::error::Error;
use std::fs::File;
use std::io::Read;
use std::io::Seek;
use std::io::SeekFrom;
use std::io::Write;
use std::path::PathBuf;

#[derive(Parser)]
#[clap()]
struct Args {
    infile: PathBuf,
    outfile: PathBuf,
}

struct ChunkHeader {
    chunk_signature: u32,
    size: u32,
}

fn read_chunk_header(infile: &mut File) -> Result<ChunkHeader, std::io::Error> {
    let mut buffer: [u8; 4] = [0; 4];

    infile.read_exact(&mut buffer)?;
    let chunk_signature = u32::from_le_bytes(buffer);

    infile.read_exact(&mut buffer)?;
    let size = u32::from_le_bytes(buffer);

    Ok(ChunkHeader {
        chunk_signature,
        size,
    })
}

#[derive(Debug)]
enum Wav2RawError {
    CantReadHeader,
    InvalidHeader,
}

impl std::fmt::Display for Wav2RawError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match &self {
            Wav2RawError::CantReadHeader => write!(f, "Can't read from input file"),
            Wav2RawError::InvalidHeader => write!(f, "Invalid header found in file"),
        }
    }
}

impl std::error::Error for Wav2RawError {}

const RIFF_MAGIC_NUM: u32 = 1179011410;
const WAVE_MAGIC_NUM: u32 = 1163280727;
const CHUNK_MAGIC_NUM: u32 = 1635017060;

fn read_header(infile: &mut File) -> Result<(), Wav2RawError> {
    let mut buffer: [u8; 12] = [0; 12];
    let riff;
    let wave;

    match infile.read_exact(&mut buffer) {
        Ok(()) => {
            riff = u32::from_le_bytes(<[u8; 4]>::try_from(&buffer[..4]).unwrap());
            wave = u32::from_le_bytes(<[u8; 4]>::try_from(&buffer[8..]).unwrap());
        }
        Err(_) => {
            return Err(Wav2RawError::CantReadHeader);
        }
    }

    match (riff, wave) {
        (RIFF_MAGIC_NUM, WAVE_MAGIC_NUM) => Ok(()),
        _ => Err(Wav2RawError::InvalidHeader),
    }
}

fn copy_data(infile: &mut File, outfile: &mut File) -> Result<(), std::io::Error> {
    while let Ok(chunkheader) = read_chunk_header(infile) {
        match chunkheader {
            // data chunk -> copy to outfile
            ChunkHeader {
                chunk_signature: CHUNK_MAGIC_NUM,
                size,
            } => {
                let mut buffer = vec![0; size as usize];
                infile.read_exact(&mut buffer[..size as usize])?;
                outfile.write_all(&buffer[..size as usize])?;
            }
            // any other type of chunk -> ignore and advance file
            ChunkHeader {
                chunk_signature: _,
                size,
            } => {
                infile.seek(SeekFrom::Current(size as i64))?;
            }
        }
    }

    Ok(())
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = Args::parse();

    println!("{} {}", args.infile.display(), args.outfile.display());

    let mut infile = File::open(args.infile)?;
    let mut outfile = File::create_new(args.outfile)?;

    read_header(&mut infile)?;

    copy_data(&mut infile, &mut outfile)?;

    Ok(())
}
