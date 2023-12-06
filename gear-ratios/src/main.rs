use regex::Regex;
use std::{
    collections::HashMap,
    fs::File,
    io::{BufRead, BufReader},
    time::Instant,
};

fn main() -> std::io::Result<()> {
    let start = Instant::now();
    println!("Part 1: {:#?}", part_1());
    let done = start.elapsed().as_secs_f64();
    println!("Done in {:#?} secs", done);
    Ok(())
}

fn part_1() -> u128 {
    let (symbols_map, numbers_map) =
        precompute_symbol_and_numbers_map("input.txt").expect("Failed to compute both maps");
    let mut sol: u128 = 0;
    for key in numbers_map.keys() {
        if let Some(ranges) = numbers_map.get(key) {
            ranges
                .iter()
                .for_each(|range| sol += check_in_symbol_range(range, &symbols_map, key))
        }
    }
    sol
}

fn check_in_symbol_range(
    range: &(i32, (i32, i32)),
    map: &HashMap<String, Vec<(i32, i32)>>,
    num: &u32,
) -> u128 {
    let (num_row, (col_start, col_end)) = range;
    let (row_start, row_end) = (num_row, num_row);
    let mut acc: u128 = 0;
    for (_key, value) in map {
        value.iter().for_each(|inter| {
            let (sym_row_start, sym_row_end) = (inter.0 - 1, inter.0 + 1);
            let (sym_col_start, sym_col_end) = (inter.1 - 1, inter.1 + 1);
            if !(*col_end < sym_col_start
                || sym_col_end < *col_start
                || *row_end < sym_row_start
                || sym_row_end < *row_start)
            {
                println!("Adding {:#?} im range of {:#?}", num, _key);
                acc += *num as u128;
            }
        })
    }

    acc
}

fn find_numbers_in_line(
    line: &String,
    row: i32,
    map: &mut HashMap<u32, Vec<(i32, (i32, i32))>>,
) -> std::io::Result<()> {
    let mut buf = String::new();
    let mut start: Option<i32> = None;
    for (pos, ch) in line.chars().enumerate() {
        if ch.is_numeric() && start.is_none() {
            start = Some(pos as i32);
            buf.push(ch)
        } else if !ch.is_numeric() {
            match u32::from_str_radix(&buf, 10) {
                Ok(number) => {
                    map.entry(number)
                        .and_modify(|e| e.push((row, (start.unwrap(), (pos - 1) as i32))))
                        .or_insert(vec![(row, (start.unwrap(), (pos - 1) as i32))]);
                    start = None;
                    buf.clear();
                }
                Err(_) => {
                    start = None;
                }
            }
        } else {
            buf.push(ch);
        }
    }
    Ok(())
}

fn find_symbols_in_line(
    line: &String,
    row: i32,
    map: &mut HashMap<String, Vec<(i32, i32)>>,
) -> std::io::Result<()> {
    let lang = Regex::new(r"[a-zA-Z0-9.]").expect("Failed to build the regex.");
    let mut col = 0;
    for ch in line.chars() {
        let char_as_string = ch.to_string();
        if lang.is_match(&char_as_string) {
            col += 1;
            continue;
        }
        map.entry(char_as_string)
            .and_modify(|e| e.push((row.clone(), col)))
            .or_insert(vec![(row, col)]);

        col += 1;
    }
    Ok(())
}

fn precompute_symbol_and_numbers_map(
    file_name: &str,
) -> std::io::Result<(
    HashMap<String, Vec<(i32, i32)>>,
    HashMap<u32, Vec<(i32, (i32, i32))>>,
)> {
    let file = File::open(file_name)?;
    let buffer = BufReader::new(file);
    let mut current: i32 = 0;
    let mut map_symbols: HashMap<String, Vec<(i32, i32)>> = HashMap::new();
    let mut map_numbers: HashMap<u32, Vec<(i32, (i32, i32))>> = HashMap::new();
    buffer.lines().for_each(|line| {
        let line_ok = line.expect("Failed to find a line");
        let _ = find_symbols_in_line(&line_ok, current, &mut map_symbols);
        let _ = find_numbers_in_line(&line_ok, current, &mut map_numbers);
        current += 1;
    });
    Ok((map_symbols, map_numbers))
}

mod tests {
    #[test]
    fn test_part_1() -> std::io::Result<()> {
        assert_eq!(4361, crate::part_1());
        Ok(())
    }
}
