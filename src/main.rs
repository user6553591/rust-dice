extern crate rand;

use rand::Rng;

fn main() {
    let mut rng = rand::os::OsRng::new().unwrap_or_else(|e| {
        println!("Could not initialize random number generator: {}", e);
        std::process::exit(1);
    });
    println!("{:?}", rng.gen_range(1, 7));
}
