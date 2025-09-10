fn main() {}

#[derive(Debug)]
enum Elem {
    X,
    Value(isize),
    Plus,
    Minus,
}

#[cfg(test)]
fn eval_algebra(input: &str) -> isize {
    let elems: Vec<_> = input
        .split_whitespace()
        .filter_map(|elem| match elem {
            "x" => Some(Elem::X),
            "=" => None,
            "+" => Some(Elem::Plus),
            "-" => Some(Elem::Minus),
            elem => Some(Elem::Value(elem.parse().unwrap())),
        })
        .collect();

    // `elems` now contains 4 elements :
    // - lhs : value (or x)
    // - operator : Plus or Minus
    // - rhs : value (or x)
    // - result

    let lhs = &elems[0];
    let operator = &elems[1];
    let rhs = &elems[2];
    let res = match &elems[3] {
        Elem::Value(value) => *value,
        _ => unreachable!(),
    };

    println!("lhs={lhs:?} operator={operator:?} rhs={rhs:?} res={res}");

    let x =
        (res - match (lhs, rhs, operator) {
            (Elem::X, Elem::Value(value), Elem::Minus) => -value,
            (Elem::X, Elem::Value(value), _) => *value,
            (Elem::Value(value), Elem::X, _) => *value,
            (_, _, _) => unreachable!(),
        }) * match (operator, lhs) {
            (Elem::Minus, Elem::X) => 1,
            (Elem::Minus, _) => -1,
            (Elem::Plus, _) => 1,
            _ => unreachable!(),
        };

    x
}

#[test]
fn test_function() {
    assert_eq!(eval_algebra("2 + x = 19"), 17);
    assert_eq!(eval_algebra("4 - x = 1"), 3);
    assert_eq!(eval_algebra("x + 10 = 53"), 43);
    assert_eq!(eval_algebra("-23 + x = -20"), 3);
    assert_eq!(eval_algebra("10 + x = 5"), -5);
    assert_eq!(eval_algebra("-49 - x = -180"), 131);
    assert_eq!(eval_algebra("x - 22 = -56"), -34);
}
