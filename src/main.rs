extern crate rand;
extern crate clap;

use rand::Rng;
use std::str::FromStr;

const NAME: &'static str = env!("CARGO_PKG_NAME");
const DESCRIPTION: &'static str = env!("CARGO_PKG_DESCRIPTION");
const VERSION: &'static str = env!("CARGO_PKG_VERSION");

fn main() {
    let matches = clap::App::new(NAME)
        .version(VERSION)
        .author("user6553591 <reddit.com/u/user6553591>")
        .about(DESCRIPTION)
        .arg(clap::Arg::with_name("sides")
            .help("The nuber of sides on the die")
            .takes_value(true)
            .short("s")
            .long("sides")
            .multiple(false))
        .get_matches();

    let sides = u8::from_str(matches.value_of("sides").unwrap_or("6")).unwrap_or_else(|_| {
        println!("Type mismatch: Exiting.");
        std::process::exit(1);
    });

    let mut rng = rand::os::OsRng::new().unwrap_or_else(|_| {
        println!("Could not initialize random number generator: Exiting.");
        std::process::exit(1);
    });

    println!("{} pips on top side.", rng.gen_range(0, sides) + 1);
}
