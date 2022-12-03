use std::{str::FromStr, time::Instant};

use strum::{EnumString, ParseError};

#[derive(EnumString, PartialEq)]
enum Move {
    #[strum(serialize = "A", serialize = "X")]
    Rock,
    #[strum(serialize = "B", serialize = "Y")]
    Paper,
    #[strum(serialize = "C", serialize = "Z")]
    Scissors,
}

enum Should {
    Win,
    Lose,
    Draw,
}

struct Elves {
    rounds: Vec<(Move, Move)>,
}

impl FromStr for Elves {
    type Err = ParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let rounds: Vec<(Move, Move)> = s
            .split("\n")
            .map(|round| {
                let (opp, elv) = round.split_once(" ").unwrap();
                let opp: Move = opp.trim().parse().unwrap();
                let elv: Move = elv.trim().parse().unwrap();

                (opp, elv)
            })
            .collect();
        Ok(Self { rounds })
    }
}

impl Elves {
    fn round_point(opp: &Move, elf: &Move) -> u32 {
        let won = match (opp, elf) {
            (Move::Rock, Move::Rock) => 3,
            (Move::Paper, Move::Paper) => 3,
            (Move::Scissors, Move::Scissors) => 3,

            (Move::Rock, Move::Paper) => 6,
            (Move::Paper, Move::Scissors) => 6,
            (Move::Scissors, Move::Rock) => 6,

            (Move::Rock, Move::Scissors) => 0,
            (Move::Paper, Move::Rock) => 0,
            (Move::Scissors, Move::Paper) => 0,
        };
        let used: u32 = match elf {
            Move::Rock => 1,
            Move::Paper => 2,
            Move::Scissors => 3,
        };
        won + used
    }

    pub fn part1_total_score(&self) -> u32 {
        self.rounds
            .iter()
            .fold(0, |acc, (opp, elf)| acc + Elves::round_point(opp, elf))
    }

    pub fn part2_total_score(&self) -> u32 {
        self.rounds.iter().fold(0, |acc, (opp, elf)| {
            let should = match &elf {
                Move::Rock => Should::Lose,
                Move::Paper => Should::Draw,
                Move::Scissors => Should::Win,
            };
            let elf_move = match (should, opp) {
                (Should::Win, Move::Rock) => Move::Paper,
                (Should::Win, Move::Paper) => Move::Scissors,
                (Should::Win, Move::Scissors) => Move::Rock,
                (Should::Lose, Move::Rock) => Move::Scissors,
                (Should::Lose, Move::Paper) => Move::Rock,
                (Should::Lose, Move::Scissors) => Move::Paper,
                (Should::Draw, Move::Rock) => Move::Rock,
                (Should::Draw, Move::Paper) => Move::Paper,
                (Should::Draw, Move::Scissors) => Move::Scissors,
            };
            acc + Elves::round_point(opp, &elf_move)
        })
    }
}

#[tokio::main]
async fn main() {
    let input = lib::get_input(2022, 2).await;

    let now = Instant::now();
    let elves: Elves = input.parse().unwrap();
    println!("Parse done: {:?}", now.elapsed());

    let now = Instant::now();
    let total = elves.part1_total_score();
    println!("Part 1: {:?}", now.elapsed());
    println!("Total score: {}", total);

    let now = Instant::now();
    let total = elves.part2_total_score();
    println!("Part 2: {:?}", now.elapsed());
    println!("Total score: {}", total);
}

#[cfg(test)]
mod test {
    use crate::Elves;

    fn get_input() -> String {
        r#"
		A Y
		B X
		C Z
		"#
        .trim()
        .to_string()
    }
    #[test]
    fn parse_elves() {
        get_input().parse::<Elves>().unwrap();
    }

    #[test]
    fn part_1_example() {
        let elves = get_input().parse::<Elves>().unwrap();
        assert_eq!(elves.part1_total_score(), 15);
    }
}
