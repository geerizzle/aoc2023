use std::{
    fs::File,
    io::{BufRead, BufReader},
};

fn main() {
    let (sol_1, sol_2) = solve_for_file("input.txt").unwrap();
    println!("Sol 1: {}", sol_1);
    println!("Sol 2: {}", sol_2);
}

fn part_1(line: &String) -> u32 {
    let (win, my) = line.split_at(line.find("|").unwrap());
    let (_, win) = win.split_at(win.find(":").unwrap());
    let win: Vec<&str> = win.trim()[2..].split(" ").collect();
    let my: Vec<&str> = my.trim()[1..]
        .split(" ")
        .filter(|num| !num.is_empty())
        .collect();
    let my: Vec<&&str> = my.iter().filter(|el| win.contains(el)).collect();
    if my.len() == 0 as usize {
        return 0;
    } else {
        2u32.pow(my.len() as u32 - 1)
    }
}

fn solve_for_file(file_name: &str) -> std::io::Result<(u128, u128)> {
    let file = File::open(file_name)?;
    let buffer = BufReader::new(file);
    let mut sol_1: u128 = 0;
    let sol_2: u128 = 0;
    buffer.lines().for_each(|line| {
        let line_as_input: String = line.expect("Failed to retrieve line");
        sol_1 += part_1(&line_as_input) as u128
    });
    Ok((sol_1, sol_2))
}

mod tests {
    #[test]
    fn test_part1() -> std::io::Result<()> {
        assert_eq!(
            13 as u128,
            crate::solve_for_file("input_test.txt").unwrap().0
        );
        Ok(())
    }
}
