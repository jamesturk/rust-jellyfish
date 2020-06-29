use unicode_segmentation::UnicodeSegmentation;

pub fn match_rating_codex(s: &str) -> String {
    // match rating only really makes sense on strings
    let s = &s.to_uppercase()[..];
    let v = UnicodeSegmentation::graphemes(s, true).collect::<Vec<&str>>();
    let mut codex = String::new();
    let mut prev = "~tmp~";

    for (i, c) in v.iter().enumerate() {
        let vowel = *c == "A" || *c == "E" || *c == "I" || *c == "O" || *c == "U";
        // not a space || starting char & vowel || non-double consonant
        if *c != " " && (i == 0 && vowel) || (!vowel && *c != prev) {
            codex.push_str(*c);
        }
        prev = c;
    }

    if codex.len() > 6 {
        let mut newcodex = String::new();
        newcodex.push_str(codex.get(..3).unwrap());
        newcodex.push_str(codex.get(codex.len() - 3..).unwrap());
        return newcodex;
    }

    return codex;
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::testutils::testutils;
    #[test]
    fn test_match_rating() {
        testutils::test_str_func("testdata/match_rating_codex.csv", match_rating_codex);
    }
}
