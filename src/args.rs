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
use std::path::PathBuf;
use structopt::StructOpt;

#[derive(StructOpt)]
pub struct Pngme {
    #[structopt(subcommand)]
    pub cmd: Command,
}

#[derive(StructOpt)]
pub enum Command {
    Encode(Encode),
    Decode(Decode),
    Remove(Remove),
    Print(Print),
}

#[derive(StructOpt)]
/// Encodes a message into a PNG file
pub struct Encode {
    /// A PNG file
    #[structopt(parse(from_os_str))]
    pub file: PathBuf,
    /// A 4 character chunk type code
    pub chunk_type: String,
    /// The message you wish to hide in the PNG file
    pub message: String,
    /// A output file
    #[structopt(parse(from_os_str))]
    pub output: Option<PathBuf>,
}

#[derive(StructOpt)]
/// Decodes a PNG that may have a hidden message
pub struct Decode {
    /// A PNG file
    #[structopt(parse(from_os_str))]
    pub file: PathBuf,
    /// A 4 character chunk type code
    pub chunk_type: String,
}

#[derive(StructOpt)]
/// Removes a chunk from a PNG file
pub struct Remove {
    /// A PNG file
    #[structopt(parse(from_os_str))]
    pub file: PathBuf,
    /// The chunk type you wish to remove
    pub chunk_type: String,
}

#[derive(StructOpt)]
/// Prints all of the chunks in a PNG file
pub struct Print {
    /// A PNG file
    #[structopt(parse(from_os_str))]
    pub file: PathBuf,
}
