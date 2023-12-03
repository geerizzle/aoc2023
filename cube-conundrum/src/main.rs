use std::{
    fs::File,
    io::{BufRead, BufReader},
};

fn part1(line: &String) {
    todo!();
}

fn part2(line: &String) {
    todo!();
}

fn main() -> std::io::Result<()> {
    let file = File::open("input_test.txt")?;
    let buffer = BufReader::new(file);
    buffer.lines().for_each(|line| {
        let line_as_input: String = line.expect("Failed to retrieve line");
    });
    Ok(())
}
