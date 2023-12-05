use std::{
    cmp::max,
    collections::HashMap,
    fs::File,
    io::{BufRead, BufReader},
};

fn get_id_and_sets(line: &String) -> (u32, Vec<&str>) {
    let game_id: u32 = line
        .split(":")
        .find_map(|split| split.split(" ").find_map(|id| id.parse::<u32>().ok()))
        .expect("Failed to retrieve the game id.");
    let sets: Vec<&str> = line
        .split(":")
        .nth(1)
        .expect("No sets provided")
        .split(";")
        .collect();

    (game_id, sets)
}

fn part1(line: &String) -> u32 {
    let allowed: HashMap<&str, u32> = HashMap::from([("red", 12), ("green", 13), ("blue", 14)]);
    let (id, sets) = get_id_and_sets(line);
    let mut invalid: i32 = 0;
    for set in &sets {
        set.split(",").for_each(|pull| {
            let trimmed = pull.trim();
            let (value, key) = trimmed.split_at(trimmed.find(" ").unwrap());
            let value_as_int =
                u32::from_str_radix(value, 10).expect("Failed to convert value to int");
            if value_as_int > *allowed.get(key.trim()).unwrap() {
                invalid -= 999;
            }
        });
        if invalid < 0 {
            return 0;
        }
    }
    id as u32
}

fn part2(line: &String) -> u32 {
    let (_, sets) = get_id_and_sets(line);
    let mut maxs: HashMap<&str, u32> = HashMap::from([("red", 0), ("blue", 0), ("green", 0)]);
    for set in &sets {
        set.split(",").for_each(|pull| {
            let trimmed = pull.trim();
            let (value, color) = trimmed.split_at(trimmed.find(" ").unwrap());
            let value_as_int =
                u32::from_str_radix(value, 10).expect("Failed to convert value to int");
            println!("Color {}", color);
            maxs.insert(color.trim(), max(maxs[color.trim()], value_as_int));
        })
    }

    maxs["red"] * maxs["blue"] * maxs["green"]
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

fn main() -> std::io::Result<()> {
    println!(
        "Solution part 1: {}",
        apply_fn_to_input(&part1, "input.txt").unwrap()
    );
    println!(
        "Solution part 2: {}",
        apply_fn_to_input(&part2, "input.txt").unwrap()
    );

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::{apply_fn_to_input, part1, part2};

    #[test]
    fn test_part_1() {
        assert_eq!(8, apply_fn_to_input(&part1, "input_test.txt").unwrap())
    }
    #[test]
    fn test_part_2() {
        assert_eq!(2286, apply_fn_to_input(&part2, "input_test.txt").unwrap())
    }
}
