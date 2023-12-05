use std::{
    fs::File,
    io::{BufRead, BufReader},
};

fn main() {
    println!("Hello, world!");
}

fn apply_fn_to_input(f: &dyn Fn(&String) -> u32, file_name: &str) -> std::io::Result<u128> {
    let file = File::open(file_name)?;
    let buffer = BufReader::new(file);
    let mut sol: u128 = 0;
    buffer.lines().for_each(|line| {
        let line_as_input: String = line.expect("Failed to retrieve line");
        sol += f(&line_as_input) as u128
    });
    Ok(sol)
}

mod tests {
    #[test]
    fn test_part_1() -> std::io::Result<()> {
        Ok(())
    }
}
