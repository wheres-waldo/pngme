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

mod args;
mod chunk;
mod chunk_type;
mod commands;
mod png;

use crate::args::{Command::*, Pngme};
use anyhow::Result;
use structopt::StructOpt;

fn main() -> Result<()> {
    let app = Pngme::from_args();

    match app.cmd {
        Encode(opts) => commands::encode(opts),
        Decode(opts) => commands::decode(opts),
        Remove(opts) => commands::remove(opts),
        Print(opts) => commands::print(opts),
    }
}
