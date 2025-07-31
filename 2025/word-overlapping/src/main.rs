fn main() {}

#[cfg(test)]
fn overlap(l: &str, r: &str) -> String {
    if r.len() == 0 {
        return l.to_string();
    }

    for (match_idx, _) in l.match_indices(r.chars().nth(0).unwrap()) {
        // we try every match on l's first char
        if r.starts_with(&l[match_idx..]) {
            return l[..match_idx].to_string() + r;
        }
    }

    // no overlap, we return both words one after the other
    l.to_string() + r
}

#[test]
fn test_word_overlapping() {
    assert_eq!(overlap("sweden", "denmark"), "swedenmark");
    assert_eq!(overlap("honey", "milk"), "honeymilk");
    assert_eq!(overlap("dodge", "dodge"), "dodge");
}
