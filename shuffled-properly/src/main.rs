fn main() {}

#[cfg(test)]
fn is_shuffled_properly(input: &[usize]) -> bool {
    for window in input.windows(3) {
        // Always true but needed to make the compiler happy
        if window.len() == 3 {
            let first = window[0];
            let second = window[1];
            let third = window[2];
            if second == first + 1 && third == first + 2 {
                return false;
            } else if second == first - 1 && third == first - 2 {
                return false;
            }
        }
    }

    // default return value
    true
}

#[test]
fn test_is_shuffled_properly() {
    let input = [1, 2, 3, 5, 8, 6, 9, 10, 7, 4];
    assert_eq!(is_shuffled_properly(&input), false);

    let input = [3, 5, 1, 9, 8, 7, 6, 4, 2, 10];
    assert_eq!(is_shuffled_properly(&input), false);

    let input = [1, 5, 3, 8, 10, 2, 7, 6, 4, 9];
    assert_eq!(is_shuffled_properly(&input), true);

    let input = [1, 3, 5, 7, 9, 2, 4, 6, 8, 10];
    assert_eq!(is_shuffled_properly(&input), true);
}
