#[allow(dead_code)]

use std::path::Path;

fn main() 
{
    day_six::pre_compute_primes_multitheaded(10000, Path::new("primes(10000).json"));
}

pub mod day_one;
pub mod day_two;
pub mod day_three;
pub mod day_four;
pub mod day_five;
pub mod day_six;