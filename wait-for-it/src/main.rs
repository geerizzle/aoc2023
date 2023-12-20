use std::{fs::File, io::{BufRead, BufReader}, time::Instant};

struct Records<'a> {
    file: &'a File,
}

impl<'a> Records<'a> {
    fn new(file: &'a File) -> Self { Self { file } }

    fn part_1(&self) -> u128 {
        let (time, dist): (String, String) = self.parse_file();
        let (_, time) = time.split_at(time.find(":").unwrap() + 1);
        let (_, dist) = dist.split_at(dist.find(":").unwrap() + 1);
        let (time, dist) = (time.trim(), dist.trim());
        let time_dist = time.split_whitespace().zip(dist.split_whitespace());
        let mut sol: u128 = 1;
        time_dist.for_each(|(time, dist)| {
            sol *= self.calc_ways((time, dist));
        });
        sol
    }

    fn part_2(&self) -> u128 {
        let (time, dist): (String, String) = self.parse_file();
        let (_, time) = time.split_at(time.find(":").unwrap() + 1);
        let (_, dist) = dist.split_at(dist.find(":").unwrap() + 1);
        let (time, dist) = (time.trim(), dist.trim());
        let time = u128::from_str_radix(time.replace(" ", "").as_str(), 10).unwrap();
        let dist = u128::from_str_radix(dist.replace(" ", "").as_str(), 10).unwrap();
        let mut sol: u128 = 0;
        let mut speed: u128 = 0;
        while speed <= time {
            if speed * (time - speed) >= dist {
                sol += 1;
            }
            speed += 1;
        }
        sol
    }

    fn calc_ways(&self, race: (&'a str, &'a str)) -> u128 {
        let (time, dist): (u128, u128) = (u128::from_str_radix(race.0, 10).unwrap(), u128::from_str_radix(race.1, 10).unwrap());
        let mut speed: u128 = 0;
        let mut ways: u128 = 0;
        while speed <= time {
            if speed * (time - speed) >= dist {
                ways += 1;
            }
            speed += 1;
        }

        ways
    }

    fn parse_file(&self) -> (String, String) {
        let buffer = BufReader::new(self.file);
        let mut lines = buffer.lines();
        let time = lines.next().unwrap().unwrap();
        let distance = lines.next().unwrap().unwrap();

        (time, distance)
    }
}

fn main() -> std::io::Result<()> {
    let file = File::open("input.txt")?;
    let records = Records::new(&file);
    let now = Instant::now();
    println!("Solution for part 2: {:#?}", records.part_2());
    let time = now.elapsed();
    println!("Finished in: {:#?} microseconds", time.as_millis());
    Ok(())
}
