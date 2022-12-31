use itertools::Itertools;
use std::{collections::HashMap, str::FromStr, string::ParseError};

#[tokio::main]
async fn main() {
    let input = lib::get_input(2022, 5).await;

    let elves = input.parse::<Elves>().unwrap();
    let answer = elves.part1();
    println!("Part 1: {answer}");
    let answer = elves.part2();
    println!("Part 2: {answer}");
}

#[derive(Debug, Clone)]
struct Elves {
    stacks: Stacks,
    moves: Vec<Move>,
}

impl Elves {
    fn part1(&self) -> String {
        let mut stacks = self.stacks.clone();
        for m in self.moves.iter() {
            for i in 0..m.count {
                let p = stacks.get_mut(&m.from).unwrap().pop().expect(&format!(
                    "Error stack {} is empty at {}. \n{:?}",
                    m.from, i, m
                ));
                stacks.get_mut(&m.to).unwrap().push(p);
            }
        }
        let mut s = String::new();
        for (_, v) in stacks.iter().sorted_by(|a, b| Ord::cmp(a.0, b.0)) {
            s.push_str(v.last().unwrap());
        }
        s
    }

    fn part2(&self) -> String {
        let mut stacks = self.stacks.clone();
        for m in self.moves.iter() {
            let stack = stacks.get_mut(&m.from).unwrap();
            debug_assert_eq!(((stack.len() - m.count)..stack.len()).len(), m.count);
            let mut slice = stack
                .splice((stack.len() - m.count)..stack.len(), [])
                .collect::<Vec<_>>();
            stacks.get_mut(&m.to).unwrap().append(&mut slice);
        }

        let mut s = String::new();
        for (_, v) in stacks.iter().sorted_by(|a, b| Ord::cmp(a.0, b.0)) {
            s.push_str(v.last().unwrap());
        }
        s
    }
}

type Stacks = HashMap<usize, Vec<String>>;

impl FromStr for Elves {
    type Err = ParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (s, m) = s.split_once("\n\n").unwrap();
        let mut stacks: Stacks = HashMap::default();
        {
            let mut s = s.rsplit("\n");
            let indexes = s.next().unwrap().split("").collect::<Vec<_>>();
            for (_, stack) in s.enumerate() {
                let s = stack.split("");
                let v = s
                    .enumerate()
                    .filter_map(|(n, s)| {
                        let c = *indexes.get(n).unwrap();
                        if c == "" || c == " " {
                            None
                        } else {
                            Some(s)
                        }
                    })
                    .collect::<Vec<_>>();
                v.iter().enumerate().for_each(|(n, s)| {
                    if s != &" " {
                        if let Some(v) = stacks.get_mut(&(n + 1)) {
                            v.push(s.to_string());
                        } else {
                            stacks.insert(n + 1, vec![s.to_string()]);
                        }
                    }
                });
            }
        }
        let mut moves: Vec<Move> = vec![];
        {
            for m in m.split("\n") {
                let mut v = m.split(" ");
                let mv = Move {
                    count: v.nth(1).unwrap().parse().unwrap(),
                    from: v.nth(1).unwrap().parse().unwrap(),
                    to: v.nth(1).unwrap().parse().unwrap(),
                };
                moves.push(mv);
            }
        }
        Ok(Elves { stacks, moves })
    }
}

#[derive(Debug, Clone, Copy)]
struct Move {
    count: usize,
    from: usize,
    to: usize,
}
