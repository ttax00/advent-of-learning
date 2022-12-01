use std::{str::FromStr, string::ParseError, time::Instant};

use lib;

struct Elves {
    calories: Vec<u32>,
}

impl Elves {
    pub fn get_fat(&self) -> &u32 {
        return self.calories.get(0).unwrap();
    }
    pub fn get_fat_sum(&self, top: usize) -> u32 {
        let mut sum = 0;
        for i in 0..top {
            let calories = self.calories.get(i).unwrap();
            sum += calories;
        }
        sum
    }
}

impl FromStr for Elves {
    type Err = ParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut calories = s
            .trim()
            .split("\n\n")
            .map(|elf| {
                let acc = elf
                    .split("\n")
                    .fold(0, |acc, itm| acc + itm.parse::<u32>().unwrap());
                acc
            })
            .collect::<Vec<u32>>();
        calories.sort_by(|a, b| b.cmp(&a));
        Ok(Elves { calories })
    }
}

#[tokio::main]
async fn main() {
    let now = Instant::now();
    let input = lib::get_input(2022, 1).await;
    println!("Fetch input taken: {:?} ", now.elapsed());
    let now = Instant::now();
    let elves = input.parse::<Elves>().unwrap();
    println!("Parse input taken: {:?} ", now.elapsed());
    let now = Instant::now();
    let calories = elves.get_fat();
    println!("Taken: {:?} ", now.elapsed());
    println!("Elf carry most calories: {} ", calories);
    let now = Instant::now();
    let sum = elves.get_fat_sum(3);
    println!("Taken: {:?} ", now.elapsed());
    println!("Sum of top 3 elves: {}", sum);
}
