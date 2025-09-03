fn main() {}

#[cfg(test)]
const LETTERS: [char; 26] = [
    'A', 'B', 'C', 'D', 'E', 'F', 'G', 'H', 'I', 'J', 'K', 'L', 'M', 'N', 'O', 'P', 'Q', 'R', 'S',
    'T', 'U', 'V', 'W', 'X', 'Y', 'Z',
];

#[cfg(test)]
fn convert_recursive(num: usize, mut res: String) -> (usize, String) {
    println!("num: {num}, res: '{res}'");
    if num > LETTERS.len() {
        let rem = num.rem_euclid(LETTERS.len());
        println!("rem: {rem}");
        if rem == 0 {
            res.push('Z');
            convert_recursive(num.div_euclid(LETTERS.len()) - 1, res)
        } else {
            res.push(LETTERS[rem - 1]);
            convert_recursive(num.div_euclid(LETTERS.len()), res)
        }
    } else {
        res.push(LETTERS[num - 1]);
        (0, res)
    }
}

#[cfg(test)]
fn convert_to_title(input: usize) -> String {
    let (_, res) = convert_recursive(input, String::new());
    res.chars().rev().collect()
}

#[test]
fn test_function() {
    assert_eq!(convert_to_title(1), "A");
    assert_eq!(convert_to_title(18), "R");
    assert_eq!(convert_to_title(26), "Z");
    assert_eq!(convert_to_title(27), "AA");
    assert_eq!(convert_to_title(28), "AB");
    assert_eq!(convert_to_title(48), "AV");
    assert_eq!(convert_to_title(51), "AY");
    assert_eq!(convert_to_title(52), "AZ");
    assert_eq!(convert_to_title(53), "BA");
    assert_eq!(convert_to_title(701), "ZY");
    assert_eq!(convert_to_title(229704), "MATT");
    assert_eq!(convert_to_title(209380622941), "ZATOICHI");
}
