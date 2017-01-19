extern crate rand;
extern crate clap;

use rand::Rng;
use std::str::FromStr;

const VERSION: &'static str = env!("CARGO_PKG_VERSION");

fn main() {
    let matches = clap::App::new("rust-dice")
        .version(VERSION)
        .author("user6553591 <reddit.com/u/user6553591>")
        .about("A simple CLI program that simulates dice")
        .arg(clap::Arg::with_name("sides")
            .help("The nuber of sides on the die")
            .takes_value(true)
            .short("s")
            .long("sides")
            .multiple(false))
        .get_matches();

    let sides = u8::from_str(matches.value_of("sides").unwrap_or("6")).unwrap();

    let mut rng = rand::os::OsRng::new().unwrap_or_else(|e| {
        println!("Could not initialize random number generator: {}", e);
        std::process::exit(1);
    });

    println!("{} pips on top side.", rng.gen_range(0, sides) + 1);
}
