use std::cmp;
use unicode_segmentation::UnicodeSegmentation;

fn vec_jaro_or_winkler<T: PartialEq>(
    s1: &Vec<T>,
    s2: &Vec<T>,
    winklerize: bool,
    long_tolerance: bool,
) -> f64 {
    let s1_len = s1.len();
    let s2_len = s2.len();

    if s1_len == 0 || s2_len == 0 {
        return 0.0;
    }

    let min_len = cmp::max(s1_len, s2_len);
    let search_range = (min_len / 2) - 1;

    let mut s1_flags = vec![false; s1_len];
    let mut s2_flags = vec![false; s2_len];
    let mut common_chars = 0;

    // looking only within search range, count & flag matched pairs
    for (i, s1_ch) in s1.iter().enumerate() {
        // avoid underflow on i - search_range
        let low = if search_range >= i { 0 } else { i - search_range };
        let hi = cmp::min(i + search_range, s2_len - 1);
        for j in low..hi + 1 {
            if !s2_flags[j] && s2[j] == *s1_ch {
                s1_flags[i] = true;
                s2_flags[j] = true;
                common_chars += 1;
                break;
            }
        }
    }

    // no characters match
    if common_chars == 0 {
        return 0.0;
    }

    // count transpositions
    let mut k = 0;
    let mut trans_count = 0;
    for (i, s1_f) in s1_flags.iter().enumerate() {
        if *s1_f {
            let mut j = k;
            while j < s2_len {
                if s2_flags[j] {
                    k = j + 1;
                    break;
                }
                j += 1;
            }
            if s1[i] != s2[j] {
                trans_count += 1
            }
        }
    }
    // need to do floor division then cast to float
    let trans_count = (trans_count / 2) as f64;
    let common_charsf = common_chars as f64;
    let s1_lenf = s1_len as f64;
    let s2_lenf = s2_len as f64;

    // adjust for similarities in nonmatched characters
    let mut weight = (common_charsf / s1_lenf
        + common_charsf / s2_lenf
        + (common_charsf - trans_count) / common_charsf)
        / 3.0;

    // winkler moddification: continue to boost similar strings
    if winklerize && weight > 0.7 && s1_len > 3 && s2_len > 3 {
        let mut i = 0;
        let j = cmp::min(min_len, 4);
        while i < j && s1[i] == s2[i] {
            // TODO: also had s1[i] in Python, necessary?
            i += 1;
        }
        if i > 0 {
            weight += (i as f64) * 0.1 * (1.0 - weight);
        }

        // optional adjustment for long strings
        // after agreeing beginning items, at least two or more must agree
        // and agreed items must be more than half of remaining items
        if long_tolerance && min_len > 4 && common_chars > i + 1 && 2 * common_chars >= min_len + i
        {
            weight += (1.0 - weight)
                * (common_chars - i - 1) as f64
                * (s1_len + s2_len - i * 2 + 2) as f64;
        }
    }

    return weight;
}

pub fn vec_jaro_similarity<T: PartialEq>(s1: &Vec<T>, s2: &Vec<T>) -> f64 {
    vec_jaro_or_winkler(s1, s2, false, false)
}

pub fn vec_jaro_winkler_similarity<T: PartialEq>(
    s1: &Vec<T>,
    s2: &Vec<T>,
    long_tolerance: bool,
) -> f64 {
    vec_jaro_or_winkler(s1, s2, true, long_tolerance)
}

pub fn jaro_similarity(s1: &str, s2: &str) -> f64 {
    let us1 = UnicodeSegmentation::graphemes(s1, true).collect::<Vec<&str>>();
    let us2 = UnicodeSegmentation::graphemes(s2, true).collect::<Vec<&str>>();

    vec_jaro_similarity(&us1, &us2)
}

pub fn jaro_winkler_similarity(s1: &str, s2: &str, long_tolerance: bool) -> f64 {
    let us1 = UnicodeSegmentation::graphemes(s1, true).collect::<Vec<&str>>();
    let us2 = UnicodeSegmentation::graphemes(s2, true).collect::<Vec<&str>>();

    vec_jaro_winkler_similarity(&us1, &us2, long_tolerance)
}


#[cfg(test)]
mod test {
    use super::*;
    use crate::testutils;
    #[test]
    fn test_jaro() {
        testutils::test_similarity_func("testdata/jaro_distance.csv", jaro_similarity);
    }
}
