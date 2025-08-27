fn main() {}

#[cfg(test)]
fn is_valid_hex_code(code: &str) -> bool {
    if code.len() != 7 || !code.starts_with("#") {
        return false;
    }

    code[1..].chars().all(|c| match c {
        '0'..='9' | 'a'..='f' | 'A'..='F' => true,
        _ => false,
    })
}

#[test]
fn test_function() {
    assert_eq!(is_valid_hex_code("1CD5C5C"), false);
    assert_eq!(is_valid_hex_code("#CD5C5C"), true);
    assert_eq!(is_valid_hex_code("#EAECEE"), true);
    assert_eq!(is_valid_hex_code("#eaecee"), true);
    assert_eq!(is_valid_hex_code("#CD5C58C"), false);
    assert_eq!(is_valid_hex_code("#CD5C5Z"), false);
    assert_eq!(is_valid_hex_code("#CD5C&C"), false);
    assert_eq!(is_valid_hex_code("CD5C5C"), false);
}
