fn main() {}

#[cfg(test)]
fn letter_combinations(input: &str) -> Vec<String> {
    use itertools::Itertools;

    let letter_lists: Vec<_> = input
        .chars()
        .map(|c| match c {
            '2' => "abc".to_string(),
            '3' => "def".to_string(),
            '4' => "ghi".to_string(),
            '5' => "jkl".to_string(),
            '6' => "mno".to_string(),
            '7' => "pqrs".to_string(),
            '8' => "tuv".to_string(),
            '9' => "wxyz".to_string(),
            c => panic!("Invalid character: {c}"),
        })
        .collect();

    letter_lists
        .iter()
        .map(|elem| elem.chars())
        .multi_cartesian_product()
        .map(|elem| elem.into_iter().collect())
        .collect()
}

#[test]
fn test_function() {
    assert_eq!(
        letter_combinations("23"),
        vec!["ad", "ae", "af", "bd", "be", "bf", "cd", "ce", "cf"]
    );
    assert_eq!(letter_combinations(""), vec![""]);
    assert_eq!(letter_combinations("2"), vec!["a", "b", "c"]);
    assert_eq!(
        letter_combinations("27"),
        vec![
            "ap", "aq", "ar", "as", "bp", "bq", "br", "bs", "cp", "cq", "cr", "cs"
        ]
    );
    assert_eq!(
        letter_combinations("234"),
        vec![
            "adg", "adh", "adi", "aeg", "aeh", "aei", "afg", "afh", "afi", "bdg", "bdh", "bdi",
            "beg", "beh", "bei", "bfg", "bfh", "bfi", "cdg", "cdh", "cdi", "ceg", "ceh", "cei",
            "cfg", "cfh", "cfi"
        ]
    );
    assert_eq!(
        letter_combinations("79"),
        vec![
            "pw", "px", "py", "pz", "qw", "qx", "qy", "qz", "rw", "rx", "ry", "rz", "sw", "sx",
            "sy", "sz"
        ]
    );
}
