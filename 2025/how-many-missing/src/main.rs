fn main() {}

#[cfg(test)]
fn how_many_missing(input: &[isize]) -> isize {
    input.last().unwrap_or(&-1) - input.first().unwrap_or(&0) - input.len() as isize + 1
}

#[test]
fn test_function() {
    assert_eq!(how_many_missing(&[1, 2, 3, 8, 9]), 4);
    assert_eq!(how_many_missing(&[1, 3]), 1);
    assert_eq!(how_many_missing(&[7, 10, 11, 12]), 2);
    assert_eq!(how_many_missing(&[1, 3, 5, 7, 9, 11]), 5);
    assert_eq!(how_many_missing(&[5, 6, 7, 8]), 0);
    assert_eq!(how_many_missing(&[]), 0);
}
