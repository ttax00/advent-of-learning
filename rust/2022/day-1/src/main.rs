use std::{str::FromStr, string::ParseError};

use lib;

struct Elves {
    calories: Vec<(usize, Vec<u32>, u32)>,
}

impl Elves {
    pub fn get_fat(&self) -> &(usize, Vec<u32>, u32) {
        return self.calories.get(0).unwrap();
    }

    pub fn get_fat_sum(&self, top: usize) -> u32 {
        let mut sum = 0;
        for i in 0..top {
            let (_, _, calories) = self.calories.get(i).unwrap();
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
            .enumerate()
            .map(|(i, elf)| {
                let v: Vec<u32> = elf.split("\n").map(|s| s.parse::<u32>().unwrap()).collect();
                let acc = v.iter().fold(0, |acc, itm| acc + itm);
                (i + 1, v, acc)
            })
            .collect::<Vec<(usize, Vec<u32>, u32)>>();
        calories.sort_by(|a, b| b.2.cmp(&a.2));
        Ok(Elves { calories })
    }
}

#[tokio::main]
async fn main() {
    let input = lib::get_input(2022, 1).await;
    let elves = input.parse::<Elves>().unwrap();
    let (index, _, calories) = elves.get_fat();
    println!("Elf {} carry most calories: {} ", index, calories);
    println!("Sum of top 3 elves: {}", elves.get_fat_sum(3));
}
