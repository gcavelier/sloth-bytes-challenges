fn main() {}

#[cfg(test)]
fn valid_string(input: &str) -> String {
    use std::collections::HashMap;

    let mut char_freq = HashMap::new();
    for c in input.chars() {
        char_freq
            .entry(c)
            .and_modify(|elem| *elem += 1)
            .or_insert(1);
    }

    dbg!(&char_freq);

    let min_value = char_freq.iter().map(|(_, v)| v).min().unwrap();
    let mut already_reduced = false;

    for (_, value) in char_freq.iter() {
        if value == min_value {
            continue;
        } else if *value == min_value + 1 && !already_reduced {
            already_reduced = true;
        } else {
            return "NO".into();
        }
    }

    "YES".into()
}

#[test]
fn test_valid_string() {
    assert_eq!(valid_string("abc"), "YES");
    assert_eq!(valid_string("abcc"), "YES");
    assert_eq!(valid_string("abccc"), "NO");
    assert_eq!(valid_string("aabbcd"), "NO");
    assert_eq!(valid_string("aabbccddeefghi"), "NO");
    assert_eq!(valid_string("abcdefghhgfedecba"), "YES");
}
