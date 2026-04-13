use std::fs::File;
use std::io::Read;
use std::io::Seek;
use std::io::SeekFrom;
use std::io::copy;

#[derive(Debug)]
pub enum Wav2RawError {
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

struct ChunkHeader {
    chunk_id: u32,
    size: u32,
}

fn read_chunk_header(infile: &mut File) -> Result<ChunkHeader, std::io::Error> {
    let mut buffer: [u8; 8] = [0; 8];

    infile.read_exact(&mut buffer)?;

    Ok(ChunkHeader {
        chunk_id: u32::from_le_bytes(<[u8; 4]>::try_from(&buffer[..4]).unwrap()),
        size: u32::from_le_bytes(<[u8; 4]>::try_from(&buffer[4..]).unwrap()),
    })
}

const RIFF_MAGIC_NUM: u32 = u32::from_le_bytes(*b"RIFF");
const WAVE_MAGIC_NUM: u32 = u32::from_le_bytes(*b"WAVE");
const DATA_MAGIC_NUM: u32 = u32::from_le_bytes(*b"data");

pub fn read_file_header(infile: &mut File) -> Result<(), Wav2RawError> {
    let mut buffer: [u8; 4] = [0; 4];

    let riff_header = match read_chunk_header(infile) {
        Ok(chunkheader) => chunkheader,
        Err(_) => return Err(Wav2RawError::CantReadHeader),
    };

    let wave = match infile.read_exact(&mut buffer) {
        Ok(()) => u32::from_le_bytes(buffer),
        Err(_) => return Err(Wav2RawError::CantReadHeader),
    };

    match (riff_header.chunk_id, wave) {
        (RIFF_MAGIC_NUM, WAVE_MAGIC_NUM) => Ok(()),
        _ => Err(Wav2RawError::InvalidHeader),
    }
}

pub fn copy_data(infile: &mut File, outfile: &mut File) -> Result<(), std::io::Error> {
    while let Ok(chunkheader) = read_chunk_header(infile) {
        match chunkheader {
            // data chunk -> copy to outfile
            ChunkHeader {
                chunk_id: DATA_MAGIC_NUM,
                size,
            } => {
                let mut chunk_data = infile.by_ref().take(size as u64);
                copy(&mut chunk_data, outfile)?;
            }
            // any other type of chunk -> ignore and advance file
            ChunkHeader { chunk_id: _, size } => {
                infile.seek(SeekFrom::Current(size as i64))?;
            }
        }
    }

    Ok(())
}
