mod hamming;
mod jaro;
mod levenshtein;
mod match_rating;
pub use hamming::{hamming_distance, vec_hamming_distance};
pub use jaro::{
    jaro_similarity, jaro_winkler_similarity, jaro_winkler_similarity_longtol, vec_jaro_similarity,
    vec_jaro_winkler_similarity, vec_jaro_winkler_similarity_longtol,
};
pub use levenshtein::{
    damerau_levenshtein_distance, levenshtein_distance, vec_damerau_levenshtein_distance,
    vec_levenshtein_distance,
};
pub use match_rating::{match_rating_codex, match_rating_comparison};
mod testutils;
