fn main() {}

#[cfg(test)]
const PLAIN_ALPHABET: &str = "abcdefghijklmnopqrstuvwxyz";

#[cfg(test)]
// NOTE: This function generates lowercase characters only
fn generate_cipher_alphabet(keyword: &str) -> String {
    let mut cipher_alphabet = keyword.to_string();

    for c in 'a'..='z' {
        if !keyword.contains(c) {
            cipher_alphabet.push(c);
        }
    }
    cipher_alphabet
}

#[cfg(test)]
fn keyword_cipher(keyword: &str, text: &str) -> String {
    let cipher_alphabet = generate_cipher_alphabet(keyword);
    let mut ciphered_text = String::new();

    for c in text.chars() {
        ciphered_text.push(
            cipher_alphabet
                .chars()
                .nth(PLAIN_ALPHABET.find(c).unwrap())
                .unwrap(),
        );
    }

    ciphered_text
}

#[test]
fn test_function() {
    assert_eq!(keyword_cipher("keyword", "abchij"), "keyabc");
    assert_eq!(keyword_cipher("purplepineapple", "abc"), "pur");
    assert_eq!(keyword_cipher("mubashir", "edabit"), "samucq");
    assert_eq!(keyword_cipher("etaoinshrdlucmfwypvbgkjqxz", "abc"), "eta");
    assert_eq!(keyword_cipher("etaoinshrdlucmfwypvbgkjqxz", "xyz"), "qxz");
    assert_eq!(
        keyword_cipher("etaoinshrdlucmfwypvbgkjqxz", "aeiou"),
        "eirfg"
    );
}
