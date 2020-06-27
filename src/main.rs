use crate::levenshtein::levenshtein_distance;
use crate::hamming::hamming_distance;

mod levenshtein;
mod hamming;

fn main() {
    println!("Hello, world!");

    println!("lev(bobsled, bobcat) = {}", levenshtein_distance("bobsled", "bobcat"));
    println!("hamming(bobsled, bobcat) = {}", hamming_distance("bobsled", "bobcat"));
}
