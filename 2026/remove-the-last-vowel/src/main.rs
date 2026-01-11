fn main() {}

#[cfg(test)]
fn remove_last_vowel_from_word(word: &str) -> String {
    if let Some((last_vowel_rev_pos, _)) =
        word.chars()
            .rev()
            .enumerate()
            .find(|(_, c)| match c.to_ascii_uppercase() {
                'A' | 'E' | 'I' | 'O' | 'U' => true,
                _ => false,
            })
    {
        let last_vowel_pos = word.len() - last_vowel_rev_pos;
        if last_vowel_pos == 0 {
            "".to_string()
        } else {
            let mut res = word[..last_vowel_pos - 1].to_string();
            if last_vowel_pos < word.len() {
                res += &word[last_vowel_pos..]
            }
            res
        }
    } else {
        word.to_string()
    }
}

#[cfg(test)]
fn remove_last_vowel(input: &str) -> String {
    let words: Vec<_> = input
        .split_whitespace()
        .map(|word| remove_last_vowel_from_word(word))
        .collect();

    words.join(" ")
}

#[test]
fn test_function() {
    assert_eq!(
        remove_last_vowel("Those who dare to fail miserably can achieve greatly."),
        "Thos wh dar t fal miserbly cn achiev gretly."
    );
    assert_eq!(
        remove_last_vowel("Love is a serious mental disease."),
        "Lov s  serios mentl diseas."
    );
    assert_eq!(
        remove_last_vowel("Get busy living or get busy dying."),
        "Gt bsy livng r gt bsy dyng."
    );
    assert_eq!(
        remove_last_vowel("If you want to live a happy life, tie it to a goal, not to people."),
        "f yo wnt t liv  hppy lif, ti t t  gol, nt t peopl."
    );
}
