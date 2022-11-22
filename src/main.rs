use anyhow::anyhow;
use anyhow::Result;
use byteorder::{LittleEndian, ReadBytesExt};
use std::{
    fs::File,
    io::{BufReader, Read},
};

use clap::Parser;

/// Extract JSON chunk from a glb file
#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    /// glb file to extract
    filename: String,
}

fn main() -> Result<()> {
    let args = Args::parse();
    let file = File::open(args.filename)?;
    let mut reader = BufReader::new(file);

    let mut magic = [0; 4];
    reader.read_exact(&mut magic)?;
    if &magic != b"glTF" {
        return Err(anyhow!("Not a glb file"));
    }

    let version = reader.read_u32::<LittleEndian>()?;
    let _length = reader.read_u32::<LittleEndian>()?;
    if version != 2 {
        return Err(anyhow!("Only glb version 2 supported"));
    }

    let chunk_length = reader.read_u32::<LittleEndian>()?;
    let chunk_type = reader.read_u32::<LittleEndian>()?;
    if chunk_type != 0x4E4F534A {
        return Err(anyhow!("Invalid first glb chunk"));
    }

    let mut json = vec![0; chunk_length as usize];
    reader.read_exact(&mut json[..])?;
    print!("{}", std::str::from_utf8(&json)?);

    Ok(())
}
