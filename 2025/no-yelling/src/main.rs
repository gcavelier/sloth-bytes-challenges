fn main() {}

#[cfg(test)]
fn no_yelling(input: &str) -> String {
    let mut res = input.to_string();

    while res.ends_with("!!") || res.ends_with("??") {
        res.pop();
    }

    res
}

#[test]
fn test_no_yelling() {
    assert_eq!(no_yelling("What went wrong?????????"), "What went wrong?");
    assert_eq!(no_yelling("Oh my goodness!!!"), "Oh my goodness!");
    assert_eq!(
        no_yelling("I just!!! can!!! not!!! believe!!! it!!!"),
        "I just!!! can!!! not!!! believe!!! it!"
    );
    assert_eq!(no_yelling("Oh my goodness!"), "Oh my goodness!");
}
