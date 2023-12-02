use std::{
    fs::File,
    io::{BufRead, BufReader},
    u128,
};

fn part_2(line: &String) -> u32 {
    let words: [&str; 9] = [
        "one", "two", "three", "four", "five", "six", "seven", "eight", "nine",
    ];

    let mut main_iter = line.chars().enumerate().filter_map(|(i, ch)| {
        if let Some(digit) = ch.to_digit(10) {
            Some(digit)
        } else {
            let sub_str = &line[i..];
            words.iter().enumerate().find_map(|(digit, digit_str)| {
                sub_str.starts_with(digit_str).then_some(digit as u32 + 1)
            })
        }
    });
    let first = main_iter.next().expect("A first digit was not found");
    let last = main_iter.last().unwrap_or(first);

    first * 10 + last
}
fn part_1(line: &String) -> u32 {
    let first: u32 = line
        .chars()
        .find(|c| c.is_numeric())
        .map(|c| c.to_digit(10).unwrap())
        .expect("Failed to find the first char");
    let last: u32 = line
        .chars()
        .rev()
        .find(|c| c.is_numeric())
        .map(|c| c.to_digit(10).unwrap())
        .expect("Failed to find the last char");

    first * 10 + last
}

fn main() -> std::io::Result<()> {
    let file = File::open("input.txt")?;
    let buffer = BufReader::new(file);
    let mut result_part1: u128 = 0;
    let mut result_part2: u128 = 0;
    buffer.lines().for_each(|line| {
        let line_as_input: String = line.expect("Failed to retrieve line");
        result_part1 += part_1(&line_as_input) as u128;
        result_part2 += part_2(&line_as_input) as u128;
    });
    println!("The result for part1: {}", result_part1);
    println!("The result for part2: {}", result_part2);
    Ok(())
}
