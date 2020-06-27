use crate::hamming::hamming_distance;
use crate::jaro::{jaro_similarity, jaro_winkler_similarity};
use crate::levenshtein::levenshtein_distance;

mod hamming;
mod jaro;
mod levenshtein;
mod testutils;

fn main() {
    println!("Hello, world!");

    println!(
        "lev(bobsled, bobcat) = {}",
        levenshtein_distance("bobsled", "bobcat")
    );
    println!(
        "hamming(bobsled, bobcat) = {}",
        hamming_distance("bobsled", "bobcat")
    );

    println!(
        "jaro(bobsled, bobcat) = {}",
        jaro_similarity("bobsled", "bobcat")
    );
    println!(
        "jarowink(bobsled, bobcat) = {}",
        jaro_winkler_similarity("bobsled", "bobcat", false)
    );
}
