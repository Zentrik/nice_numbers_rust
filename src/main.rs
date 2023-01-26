use malachite::Natural;
use malachite::num::arithmetic::traits::Pow;

use nice_numbers_rust::search_range;

use std::time::Instant;

fn main() {
    let before = Instant::now();

    let _nice_numbers: Vec<Natural> = search_range(
        Natural::from(1 as u32),
        Natural::from(10 as u32).pow(5),
        10,
        true
    );

    println!("Elapsed time: {:.4?}", before.elapsed());
}