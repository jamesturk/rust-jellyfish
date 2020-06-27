use unicode_segmentation::UnicodeSegmentation;

pub fn vec_hamming_distance<T: PartialEq>(s1: &Vec<T>, s2: &Vec<T>) -> usize {
    let (longer, shorter) = if s1.len() > s2.len() {
        (s1, s2)
    } else {
        (s2, s1)
    };

    // distance is difference in length + differing chars
    let mut distance = longer.len() - shorter.len();
    for (i, c) in shorter.iter().enumerate() {
        if *c != longer[i] {
            distance += 1
        }
    }

    return distance;
}

pub fn hamming_distance(s1: &str, s2: &str) -> usize {
    let us1 = UnicodeSegmentation::graphemes(s1, true).collect::<Vec<&str>>();
    let us2 = UnicodeSegmentation::graphemes(s2, true).collect::<Vec<&str>>();

    vec_hamming_distance(&us1, &us2)
}
