use unicode_segmentation::UnicodeSegmentation;

fn isvowel(s: &str) -> bool {
    match s {
        "A" | "E" | "I" | "O" | "U" => true,
        _ => false,
    }
}

pub fn nysiis(s: &str) -> String {
    let s = &s.to_uppercase()[..];
    let mut v = UnicodeSegmentation::graphemes(s, true).collect::<Vec<&str>>();

    if v.len() == 0 {
        return String::from("");
    }

    // step 1: handle prefixes
    if s.starts_with("MAC") {
        v[1] = "C"; // switch MAC to MCC
    } else if s.starts_with("KN") {
        v.remove(0); // strip leading K from KN
    } else if s.starts_with("K") {
        v[0] = "C"; // switch K to C
    } else if s.starts_with("PH") || s.starts_with("PF") {
        v[0] = "F";
        v[1] = "F"; // switch these to FF
    } else if s.starts_with("SCH") {
        v[1] = "S";
        v[2] = "S"; // switch SCH to SSS
    }

    // step 2: suffixes
    if s.ends_with("IE") || s.ends_with("EE") {
        v.pop();
        v.pop();
        v.push("Y");
    } else if s.ends_with("DT")
        || s.ends_with("RT")
        || s.ends_with("RD")
        || s.ends_with("NT")
        || s.ends_with("ND")
    {
        v.pop();
        v.pop();
        v.push("D");
    }

    // step 3: key starts with first character of name
    let mut key = Vec::new();
    key.push(v[0]);

    // step 4: translate remaining characters
    let mut i = 1;

    while i < v.len() {
        let chars = match v[i] {
            "E" if i + 1 < v.len() && v[i + 1] == "V" => {
                i += 1;
                vec!["A", "F"]
            }
            "A" | "E" | "I" | "O" | "U" => vec!["A"],
            "Q" => vec!["G"],
            "Z" => vec!["S"],
            "M" => vec!["N"],
            "K" => {
                if i + 1 < v.len() && v[i + 1] == "N" {
                    vec!["N"]
                } else {
                    vec!["C"]
                }
            }
            "S" if i + 2 < v.len() && v[i + 1] == "C" && v[i + 2] == "H" => {
                i += 2;
                vec!["S", "S"]
            }
            "P" if i + 1 < v.len() && v[i + 1] == "H" => {
                i += 1;
                vec!["F"]
            }
            "H" if !isvowel(v[i - 1])
                || (i + 1 < v.len() && !isvowel(v[i + 1]))
                || (i + 1 == v.len()) =>
            {
                if isvowel(v[i - 1]) {
                    vec!["A"]
                } else {
                    vec![v[i - 1]]
                }
            }
            "W" if isvowel(v[i - 1]) => vec![v[i - 1]],
            _ => vec![v[i]],
        };

        if chars.len() > 0 && chars[chars.len() - 1] != key[key.len() - 1] {
            for c in chars {
                key.push(c);
            }
        }
        println!("step 4: {:?}, {:?}", v[i], key);

        i += 1;
    }

    // step 5 remove trailing S
    if key[key.len() - 1] == "S" && key.len() > 1 {
        key.pop();
    }

    // step 6 replace AY w/ Y
    if key.ends_with(&["A", "Y"]) {
        key.remove(key.len() - 2);
    }

    // step 7 remove trailing A
    if key[key.len() - 1] == "A" && key.len() > 1 {
        key.pop();
    }

    let mut str_key = String::new();
    for k in key {
        str_key.push_str(k);
    }

    return str_key;
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::testutils::testutils;
    #[test]
    fn test_nysiis() {
        testutils::test_str_func("testdata/nysiis.csv", nysiis);
    }
}
