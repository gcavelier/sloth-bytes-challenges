fn main() {}

#[cfg(test)]
enum LetterType {
    Consonant,
    Vowel,
}

#[cfg(test)]
fn is_vowel(c: &char) -> bool {
    match c {
        'A' | 'E' | 'I' | 'O' | 'U' | 'Y' => true,
        _ => false,
    }
}

#[cfg(test)]
fn is_vowel_skewer(input: &str) -> bool {
    let input_chars: Vec<_> = input.chars().collect();

    // Checking the first character
    if let Some(c) = input_chars.first() {
        if is_vowel(c) {
            return false;
        }
    } else {
        eprintln!("input is empty");
        return false;
    }

    // Checking the last character (at this point, we know we have at least one character, so calling unwrap() is ok)
    if is_vowel(input_chars.last().unwrap()) {
        return false;
    }

    let mut spacing = 0;
    let mut max_spacing = 0;
    let mut letter_type = LetterType::Consonant;

    // We already handle the first character, so skip it
    for c in input_chars.iter().skip(1) {
        match c {
            '-' => spacing += 1,
            c => {
                if max_spacing == 0 {
                    max_spacing = spacing;
                } else {
                    if spacing > max_spacing {
                        eprintln!("spacing ({spacing}) > max_spacing ({max_spacing})");
                        return false;
                    }
                }

                match letter_type {
                    LetterType::Consonant if !is_vowel(c) => return false,
                    LetterType::Vowel if is_vowel(c) => return false,
                    LetterType::Consonant => letter_type = LetterType::Vowel,
                    LetterType::Vowel => letter_type = LetterType::Consonant,
                }
                spacing = 0;
            }
        }
    }

    // Checking if we found at least a '-'
    if max_spacing == 0 { false } else { true }
}

#[test]
fn test_valid_vowel_skewers() {
    assert_eq!(is_vowel_skewer(""), false);
    assert_eq!(is_vowel_skewer("BAC"), false);
    assert_eq!(is_vowel_skewer("B--A--N--A--N--A--S"), true);
    assert_eq!(is_vowel_skewer("A--X--E"), false); // Should start and end with a consonant.
    assert_eq!(is_vowel_skewer("C-L-A-P"), false); // Should alternate between consonants and vowels.
    assert_eq!(is_vowel_skewer("C-A-L-P"), false); // Should alternate between consonants and vowels.
    assert_eq!(is_vowel_skewer("M--A---T-E-S"), false); // Should have consistent spacing between letters.
}
