use crate::{chunk::Chunk, chunk_type::ChunkType, png::Png};
use anyhow::{bail, Context, Result};
use std::{fs, path::Path};

fn try_read_png<P: AsRef<Path>>(path: P) -> Result<Png> {
    Ok(Png::try_from(
        fs::read(path)
            .context("Failed to open PNG file")?
            .as_slice(),
    )?)
}

pub fn encode<P: AsRef<Path>>(
    path: P,
    chunk_type: ChunkType,
    message: String,
    output: Option<P>,
) -> Result<()> {
    // TODO: Maybe make this override an already existing chunk of that type
    let mut png = try_read_png(&path)?;
    png.append_chunk(Chunk::new(chunk_type, message.into_bytes()));

    let path = if let Some(out) = output { out } else { path };
    fs::write(path, png.as_bytes())?;
    Ok(())
}

pub fn decode<P: AsRef<Path>>(path: P, chunk_type: &ChunkType) -> Result<()> {
    let png = try_read_png(path)?;
    match png.chunk_by_type(chunk_type) {
        Some(chunk) => Ok(println!(
            "{}",
            chunk
                .data_as_string()
                .context("Failed to read embedded data in chunk")?
        )),
        None => bail!("no chunk with that type found"),
    }
}

pub fn remove<P: AsRef<Path>>(path: P, chunk_type: &ChunkType) -> Result<()> {
    let mut png = try_read_png(&path)?;
    png.remove_chunk(chunk_type)?;
    fs::write(path, png.as_bytes())?;
    Ok(())
}

pub fn print<P: AsRef<Path>>(path: P) -> Result<()> {
    println!("{}", try_read_png(path)?);
    Ok(())
}
