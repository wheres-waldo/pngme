// Copyright (C) 2020 Steven Waldron
//
// This file is part of pngme.
//
// pngme is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.
//
// pngme is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU General Public License for more details.
//
// You should have received a copy of the GNU General Public License
// along with pngme.  If not, see <http://www.gnu.org/licenses/>.
use crate::{
    args::{Decode, Encode, Print, Remove},
    chunk::Chunk,
    png::Png,
};
use anyhow::{Error, Result};
use std::{convert::TryFrom, fs};

/// Encodes a `Chunk` into a `Png` file
pub fn encode(opts: Encode) -> Result<()> {
    let buffer = fs::read(&opts.file)?;

    let mut png = Png::try_from(&buffer[..])?;
    let chunk = Chunk::new(opts.chunk_type.as_str(), opts.message.as_str())?;
    png.append_chunk(chunk);

    let output_file = &opts.output.unwrap_or(opts.file);
    fs::write(output_file, &png.as_bytes()[..])?;

    Ok(())
}

/// Decode a `Chunk` from a `Png` file
pub fn decode(opts: Decode) -> Result<String> {
    let buffer = fs::read(&opts.file)?;

    let png = Png::try_from(&buffer[..])?;
    let chunk = png
        .chunk_by_type(&opts.chunk_type.as_str())
        .ok_or_else(|| {
            Error::msg(format!(
                "Chunk type {} was not found in file",
                &opts.chunk_type
            ))
        })?;

    Ok(String::from_utf8(chunk.data().to_vec())?)
}

/// Removes the first `Chunk` with `chunk_type` from a `Png` file
pub fn remove(opts: Remove) -> Result<()> {
    let buffer = fs::read(&opts.file)?;

    let mut png = Png::try_from(&buffer[..])?;
    png.remove_chunk(&opts.chunk_type.as_str())?;

    fs::write(&opts.file, png.as_bytes())?;
    Ok(())
}

/// Returns a `String` that represents all of the `Chunks` in a `Png` file
pub fn print(opts: Print) -> Result<String> {
    let buffer = fs::read(&opts.file)?;

    let png = Png::try_from(&buffer[..])?;
    Ok(png
        .chunks()
        .iter()
        .map(|chunk| chunk.to_string())
        .collect::<Vec<String>>()
        .join("\n\n"))
}
