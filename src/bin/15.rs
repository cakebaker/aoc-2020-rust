fn main() {
    let mut numbers: Vec<usize> = vec![6, 19, 0, 5, 7, 13, 1];

    let start_range = numbers.len() - 1;
    let end_range = 2019;

    for i in start_range..end_range {
        match numbers[0..i].iter().rposition(|&n| n == numbers[i]) {
            Some(index) => numbers.push(i - index),
            None => numbers.push(0),
        }
    }

    println!("Result of puzzle 1: {:?}", numbers.last().unwrap());
}
