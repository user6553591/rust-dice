// rust-dice - A simple CLI program that simulates dice. Written in Rust.
// Copyright (C) 2017 user6553591
//
// This program is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.
//
// This program is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU General Public License for more details.
//
// You should have received a copy of the GNU General Public License
// along with this program.  If not, see <http://www.gnu.org/licenses/>.

extern crate rand;
extern crate clap;

use rand::Rng;
use std::str::FromStr;

const NAME: &'static str = env!("CARGO_PKG_NAME");
const AUTHORS: &'static str = env!("CARGO_PKG_AUTHORS");
const DESCRIPTION: &'static str = env!("CARGO_PKG_DESCRIPTION");
const VERSION: &'static str = env!("CARGO_PKG_VERSION");

fn main() {
    let matches: clap::ArgMatches = clap::App::new(NAME)
        .version(VERSION)
        .author(AUTHORS)
        .about(DESCRIPTION)
        .arg(clap::Arg::with_name("sides")
            .help("The nuber of sides on the die")
            .takes_value(true)
            .short("s")
            .long("sides")
            .multiple(false))
        .get_matches();

    let sides: u8 = u8::from_str(matches.value_of("sides").unwrap_or("6")).unwrap_or_else(|_| {
        println!("Type mismatch: Exiting.");
        std::process::exit(1);
    });

    let mut rng: rand::os::OsRng = rand::os::OsRng::new().unwrap_or_else(|_| {
        println!("Could not initialize random number generator: Exiting.");
        std::process::exit(1);
    });

    println!("{} pips on top side.", rng.gen_range(0, sides) + 1);
}
