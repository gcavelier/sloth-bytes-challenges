fn main() {}

#[cfg(test)]
fn maximum_seating(input: &mut [usize]) -> usize {
    let mut new_seated = 0;

    for idx in 0..input.len() {
        // The current seat is empty
        if input[idx] == 0 {
            // Checking if the next 2 seats (both left and right) are all 0

            // We first get the *valid* range of seats to check
            let range = input
                .get((idx as isize - 2).max(0) as usize..=(idx + 2).min(input.len() - 1))
                .unwrap();

            if range.iter().all(|item| *item == 0) {
                input[idx] = 1;
                new_seated += 1;
            }
        }
    }

    new_seated
}

#[test]
fn test_maximum_seating() {
    assert_eq!(maximum_seating(&mut [0, 0, 0, 1, 0, 0, 1, 0, 0, 0]), 2);
    assert_eq!(maximum_seating(&mut [0, 0, 0, 0]), 2);
    assert_eq!(maximum_seating(&mut [1, 0, 0, 0, 0, 1]), 0);
    assert_eq!(maximum_seating(&mut [0, 0, 0, 0, 0, 0, 0, 0, 0, 0]), 4);
}
