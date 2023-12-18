use std::{
    fs::File,
    io::{BufRead, BufReader, Lines}, collections::HashMap,
};

const MAPPINGS: [&str; 7]= ["soil", "fertilizer", "water", "light", "temperature", "humidity", "location"];

#[derive(Debug)]
struct Almanac<'a> {
    seeds: Seeds,
    maps: Vec<EncryptedMap<'a>>
}

#[derive(Debug)]
struct EncryptedMap<'a> {
    source_name: &'a str,
    destination_name: &'a str,
    source_dest_map: HashMap<u128, (u128, u128)>
}

#[derive(Debug)]
struct DecryptedMap<'a> {
    source_name: &'a str,
    destination_name: &'a str,
    map: (u128, u128)
}

#[derive(Debug)]
struct Seeds {
    nums: Vec<u32>,
}

impl Seeds {
    fn new(line: &str) -> Self {
        let (_, nums) = line.split_at(line.find(":").unwrap() + 1);
        let nums: Vec<u32> = nums
            .trim()
            .split(" ")
            .map(|num| u32::from_str_radix(num, 10).unwrap())
            .collect();
        Self { nums }
    }
}

impl<'a> EncryptedMap<'a> {
    pub fn new(map: &'a [String]) -> Self {
        let mut map_iter = map.iter();
        let (source_name, destination_name) = Self::get_source_dest(map_iter.next().expect("No name"));
        let mut source_dest_map: HashMap<u128, (u128, u128)> = HashMap::new();
        map_iter.for_each(|map| {
            let (key, source, len) = Self::get_range(map);
            source_dest_map.insert(key, (source, len));
        });
        Self {
            source_name,
            destination_name,
            source_dest_map
        }
    }

    fn get_source_dest(line: &String) -> (&str, &str) {
        let (source, dest) = line.split_at(line.find("-to-").unwrap());
        (source, &dest[4..dest.find(" ").unwrap()])
    }

    fn get_range(map: &String) -> (u128, u128, u128) {
        let split: Vec<u128> = map.split(" ").map(|num| u128::from_str_radix(num, 10).unwrap()).collect();
        (split[0], split[1], split[2])
    }
}

impl<'a> Almanac<'a> {
    fn new(seeds: Seeds, maps: Vec<EncryptedMap<'a>>) -> Self { Self { seeds, maps } }
    fn part_1(&self) -> u128 {
        let sol: u128 = u128::max_value();
        for (key, map) in MAPPINGS.iter().enumerate() {
            if key == 0 {
                self.generate_map_for("seeds", map);
            }
        }

        sol
    }

    fn generate_map_for(&self, arg: &str, map: &str) -> () {
        todo!()
    }

}

fn main() -> std::io::Result<()> {
    let file = File::open("input.txt")?;
    let buffer = BufReader::new(file);
    let mut lines: Lines<BufReader<File>> = buffer.lines();
    let trimmed = lines.next().expect("Buffer empty").expect("No lines left");
    let seeds: Seeds = Seeds::new(trimmed.as_str());
    let unwraped_lines: Vec<String> = lines.map(|line| line.unwrap()).collect();
    let unwraped_lines: Vec<_> = unwraped_lines.split(|line| line == "").collect();
    let mut maps: Vec<EncryptedMap> = Vec::new();
    for map in unwraped_lines {
        if map.is_empty() {
            continue;
        }

        maps.push(EncryptedMap::new(map));
    }
    println!("{:#?}", maps);
    let almanac = Almanac::new(seeds, maps);
    almanac.part_1();
    Ok(())
}
