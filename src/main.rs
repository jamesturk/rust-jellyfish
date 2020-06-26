use crate::levenshtein::levenshtein_distance;

mod levenshtein;

fn main() {
    println!("Hello, world!");

    println!("lev(bobsled, bobcat) = {}", levenshtein_distance("bobsled", "bobcat"))
}
