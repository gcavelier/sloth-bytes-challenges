fn main() {}

#[cfg(test)]
fn nom_nom(input: &[usize]) -> Vec<usize> {
    let mut res = Vec::new();
    let mut input_iter = input.into_iter();

    let mut current_num = input_iter.next().unwrap().clone(); // `input` should have at least one element

    while let Some(next_num) = input_iter.next() {
        if *next_num < current_num {
            current_num += next_num;
        } else {
            res.push(current_num);
            current_num = *next_num;
        }
    }

    res.push(current_num);
    res
}

#[test]
fn test_function() {
    assert_eq!(nom_nom(&[5, 3, 7]), [15]);
    assert_eq!(nom_nom(&[5, 3, 9]), [8, 9]);
    assert_eq!(nom_nom(&[1, 2, 3]), [1, 2, 3]);
    assert_eq!(nom_nom(&[2, 1, 3]), [3, 3]);
    assert_eq!(nom_nom(&[8, 5, 9]), [22]);
    assert_eq!(nom_nom(&[6, 5, 6, 100]), [17, 100]);
}
