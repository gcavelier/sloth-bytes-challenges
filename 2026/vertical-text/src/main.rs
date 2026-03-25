fn main() {}

#[cfg(test)]
fn vertical_txt(input: &str) -> Vec<Vec<char>> {
    let mut res = Vec::new();
    let words: Vec<_> = input.split_whitespace().collect();
    let max_nb_chars = words.iter().map(|item| item.len()).max().unwrap();

    for i in 0..max_nb_chars {
        let mut line = Vec::new();
        for j in 0..words.len() {
            let a = &words[j].chars().nth(i).unwrap_or(' ');
            line.push(a.clone());
        }
        res.push(line);
    }

    res
}

#[test]
fn test_function() {
    assert_eq!(
        vertical_txt("Holy bananas"),
        vec![
            vec!['H', 'b'],
            vec!['o', 'a'],
            vec!['l', 'n'],
            vec!['y', 'a'],
            vec![' ', 'n'],
            vec![' ', 'a'],
            vec![' ', 's'],
        ]
    );

    assert_eq!(
        vertical_txt("Hello fellas"),
        vec![
            vec!['H', 'f'],
            vec!['e', 'e'],
            vec!['l', 'l'],
            vec!['l', 'l'],
            vec!['o', 'a'],
            vec![' ', 's'],
        ]
    );
}
