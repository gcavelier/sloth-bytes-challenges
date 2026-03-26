fn main() {}

#[cfg(test)]
fn daily_temperatures(input: &[usize]) -> Vec<usize> {
    let res = input
        .iter()
        .enumerate()
        .map(|(idx, item)| {
            let (_, right) = input.split_at_checked(idx + 1).unwrap();
            let found_idx = right
                .iter()
                .enumerate()
                .filter_map(|(tmp_idx, tmp_item)| {
                    if tmp_item > item {
                        Some(tmp_idx + 1)
                    } else {
                        None
                    }
                })
                .nth(0);
            found_idx.unwrap_or(0)
        })
        .collect();

    res
}

#[test]
fn test_function() {
    assert_eq!(
        daily_temperatures(&[30, 38, 30, 36, 35, 40, 28]),
        vec![1, 4, 1, 2, 1, 0, 0]
    );
    assert_eq!(daily_temperatures(&[22, 21, 20]), vec![0, 0, 0]);
    assert_eq!(
        daily_temperatures(&[30, 38, 30, 36, 35, 40, 28]),
        vec![1, 4, 1, 2, 1, 0, 0]
    );
}
