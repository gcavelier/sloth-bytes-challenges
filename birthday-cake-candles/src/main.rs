// You are in charge of the cake for a child's birthday. It will have one candle for each year of their total age.
// They will only be able to blow out the tallest of the candles.
// Your task is to count how many candles are the tallest.

fn main() {}

#[cfg(test)]
fn birthday_cake_candles(input: Vec<usize>) -> usize {
    let mut max = 0;
    let mut count: usize = 0;

    for i in input {
        if i > max {
            max = i;
            count = 1;
        } else if i == max {
            count += 1;
        }
    }

    count
}

#[test]
fn test_birthday_cake_candles() {
    // The tallest candles are 4. There are 2 candles with this height, so the function should return 2.
    assert_eq!(birthday_cake_candles(vec![4, 4, 1, 3]), 2);
    // All candles are the same height, so all are the tallest.
    assert_eq!(birthday_cake_candles(vec![1, 1, 1, 1]), 4);
    // No candles, so nothing to blow out.
    assert_eq!(birthday_cake_candles(vec![]), 0);
    assert_eq!(birthday_cake_candles(vec![4, 4, 1, 3, 8]), 1);
}
