fn main() {}

#[cfg(test)]
fn same_upside_down(input: &str) -> bool {
    let upside_down_input: String = input
        .chars()
        .rev()
        .map(|c| match c {
            '6' => '9',
            '9' => '6',
            c => c,
        })
        .collect();

    input == upside_down_input
}

#[test]
fn test_function() {
    assert_eq!(same_upside_down("6090609"), true);
    assert_eq!(same_upside_down("9669"), false);
    assert_eq!(same_upside_down("9"), false);
    assert_eq!(same_upside_down("0"), true);
    assert_eq!(same_upside_down("60906096090609"), true);
    assert_eq!(same_upside_down("966909669"), false);
    assert_eq!(same_upside_down("96666660999999"), false);
}
