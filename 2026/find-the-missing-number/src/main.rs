fn main() {}

#[cfg(test)]
fn missing_num(input: &mut [usize]) -> usize {
    let mut missing = None;
    input.sort();

    for (idx, i) in input.iter().enumerate() {
        println!("idx={idx} i={i}");
        if idx + 1 != *i {
            missing = Some(idx + 1);
            break;
        }
    }

    dbg!(missing);

    if let Some(missing) = missing {
        missing
    } else {
        input.len() + 1
    }
}

#[test]
fn test_function() {
    assert_eq!(missing_num(&mut vec![1, 2, 3, 4, 6, 7, 8, 9, 10]), 5);
    assert_eq!(missing_num(&mut vec![7, 2, 3, 6, 5, 9, 1, 4, 8]), 10);
    assert_eq!(missing_num(&mut vec![10, 5, 1, 2, 4, 6, 8, 3, 9]), 7);
}
