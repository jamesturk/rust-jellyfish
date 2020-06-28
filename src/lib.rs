mod hamming;
mod jaro;
mod levenshtein;
pub use hamming::{hamming_distance, vec_hamming_distance};
pub use jaro::{
    jaro_similarity, jaro_winkler_similarity, jaro_winkler_similarity_longtol, vec_jaro_similarity,
    vec_jaro_winkler_similarity, vec_jaro_winkler_similarity_longtol,
};
pub use levenshtein::levenshtein_distance;
mod testutils;
