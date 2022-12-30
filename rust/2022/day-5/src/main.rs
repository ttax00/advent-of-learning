use std::{collections::HashMap, str::FromStr, string::ParseError};

#[tokio::main]
async fn main() {
    let input = lib::get_input(2022, 5).await;

    let elves = input.parse::<Elves>();
}

struct Elves {
    stacks: HashMap<u32, Vec<String>>,
    moves: Vec<Move>,
}

impl FromStr for Elves {
    type Err = ParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (stacks, moves) = s.split_once("\n\n").unwrap();
        for line in stacks.rsplit("\n") {
            let test = line.split("");
            println!("{:?}", test.collect::<Vec<_>>());
        }
        todo!()
    }
}

struct Move {
    from: u32,
    to: u32,
    count: u32,
}
