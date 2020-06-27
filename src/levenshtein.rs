use std::cmp;
use unicode_segmentation::UnicodeSegmentation;

fn range_vec(size: usize) -> Vec<usize> {
    let mut vec = Vec::new();
    let mut p: usize = 0;
    vec.resize_with(size, || {
        p += 1;
        p - 1
    });
    return vec;
}

pub fn vec_levenshtein_distance<T: PartialEq>(v1: &Vec<T>, v2: &Vec<T>) -> usize {
    let rows = v1.len() + 1;
    let cols = v2.len() + 1;

    if rows == 1 {
        return cols - 1;
    } else if cols == 1 {
        return rows - 1;
    }

    let cur = range_vec(cols);

    for r in 1..rows {
        // make a copy of the previous row so we can edit cur
        let prev = cur.to_vec();
        let mut cur = vec![0; cols];
        cur[0] = r;
        for c in 1..cols {
            // deletion cost or insertion cost
            let del_or_ins = cmp::min(prev[c] + 1, cur[c - 1] + 1);
            let edit = prev[c - 1] + (if v1[r - 1] == v2[c - 1] { 0 } else { 1 });
            cur[c] = cmp::min(del_or_ins, edit);
        }
    }

    // last element of bottom row
    return cur[cur.len() - 1];
}

pub fn levenshtein_distance(s1: &str, s2: &str) -> usize {
    if s1 == s2 {
        return 0;
    }

    let us1 = UnicodeSegmentation::graphemes(s1, true).collect::<Vec<&str>>();
    let us2 = UnicodeSegmentation::graphemes(s2, true).collect::<Vec<&str>>();

    vec_levenshtein_distance(&us1, &us2)
}
