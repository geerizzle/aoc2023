use std::{
    fs::File,
    io::{BufRead, BufReader},
};

fn part1(line: &String) -> u32 {
    todo!();
}

fn part2(line: &String) -> u32{
    todo!();
}

fn main() -> std::io::Result<()> {
    let file = File::open("input.txt")?;
    let buffer = BufReader::new(file);
    buffer.lines().for_each(|line| {
        let line_as_input: String = line.expect("Failed to retrieve line");
    });
    Ok(())
}

#[cfg(test)]
mod tests {
    use std::{fs::File, io::{BufReader, BufRead}};

    use super::{part1, part2}
    #[test]
    fn test_part_1() {
        let file = File::open("input_test.txt").unwrap();
        let buffer = BufReader::new(file);
        let mut sol = 0;
        buffer.lines().for_each(|line| {
            let line_as_input: String = line.expect("Failed to retrieve line");
            sol += part1(&line_as_input);
        });
        assert_eq!(8, sol);
    }
}
