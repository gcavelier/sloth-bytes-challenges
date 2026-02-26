fn main() {}

#[cfg(test)]
fn seven_boom(input: &[usize]) -> String {
    let count_7 = input
        .iter()
        .filter_map(
            |item| match item.to_string().chars().filter(|item| *item == '7').count() {
                0 => None,
                c => Some(c),
            },
        )
        .sum();

    if count_7 == 0 {
        "there is no 7 in the array".to_owned()
    } else {
        let mut res = String::new();
        for _ in 0..count_7 {
            res.push_str("Boom! ");
        }
        res.strip_suffix(' ').unwrap().to_owned()
    }
}

#[test]
fn test_function() {
    assert_eq!(seven_boom(&[1, 2, 3, 4, 5, 6, 7]), "Boom!");
    assert_eq!(seven_boom(&[8, 6, 33, 100]), "there is no 7 in the array");
    assert_eq!(seven_boom(&[2, 55, 60, 97, 86]), "Boom!");
    assert_eq!(seven_boom(&[7, 77, 100]), "Boom! Boom! Boom!");
}
