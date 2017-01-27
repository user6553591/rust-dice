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
// along with this program.  If not, see <https://www.gnu.org/licenses/>.

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
        .arg(clap::Arg::with_name("quiet")
            .help("Only output numbers")
            .takes_value(false)
            .short("q")
            .long("quiet")
            .multiple(false))
        .arg(clap::Arg::with_name("sides")
            .help("The number of sides on the die")
            .takes_value(true)
            .short("s")
            .long("sides")
            .multiple(false))
        .arg(clap::Arg::with_name("rolls")
            .help("The number of times the die is rolled")
            .takes_value(true)
            .short("r")
            .long("rolls")
            .multiple(false))
        .get_matches();

    let quiet = matches.is_present("quiet");

    let sides_input = matches.value_of("sides").unwrap_or("6");
    let sides: u8 = u8::from_str(sides_input).unwrap_or_else(|_| {
        println!("Error: '{}' not understood in context 'sides': Exiting.", sides_input);
        std::process::exit(1);
    });

    if (sides >= 2) == false {
        println!("Error: Die needs two or more sides: Exiting.");
        std::process::exit(1);
    }

    let rolls_input = matches.value_of("rolls").unwrap_or("1");
    let rolls: u8 = u8::from_str(rolls_input).unwrap_or_else(|_| {
        println!("Error: '{}' not understood in context 'sides': Exiting.", rolls_input);
        std::process::exit(1);
    });

    if (rolls >= 1) == false {
        println!("Error: Die needs to be rolled at least once: Exiting.");
        std::process::exit(1);
    }

    let mut rng: rand::os::OsRng = rand::os::OsRng::new().unwrap_or_else(|_| {
        println!("Error: Could not initialize random number generator: Exiting.");
        std::process::exit(1);
    });

    if quiet == false {
        println!("Rolling {} time(s), with a {} sided die...", rolls, sides);
    }
    for _ in 0..rolls {
        let number = rng.gen_range(0, sides) + 1;
        if matches.is_present("quiet") {
            println!("{}", number);
        } else {
            println!("The number {} came up.", number);
        }
    }
}
