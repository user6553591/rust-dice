extern crate rand;

use rand::Rng;

fn main() {
    let sides = 6;
    let mut rng = rand::os::OsRng::new().unwrap_or_else(|e| {
        println!("Could not initialize random number generator: {}", e);
        std::process::exit(1);
    });
    println!("{:?}", rng.gen_range(0, sides) + 1);
}
