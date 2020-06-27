use crate::levenshtein::levenshtein_distance;
use crate::hamming::hamming_distance;
use crate::jaro::{jaro_winkler_similarity, jaro_similarity};

mod levenshtein;
mod hamming;
mod jaro;

fn main() {
    println!("Hello, world!");

    println!("lev(bobsled, bobcat) = {}", levenshtein_distance("bobsled", "bobcat"));
    println!("hamming(bobsled, bobcat) = {}", hamming_distance("bobsled", "bobcat"));

    println!("jaro(bobsled, bobcat) = {}", jaro_similarity("bobsled", "bobcat"));
    println!("jarowink(bobsled, bobcat) = {}", jaro_winkler_similarity("bobsled", "bobcat", false));
}
