use std::{collections::HashSet, str::FromStr, string::ParseError, time::Instant};

struct Elves {
    data: Vec<(HashSet<u32>, HashSet<u32>)>,
}

impl FromStr for Elves {
    type Err = ParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let data: Vec<(HashSet<u32>, HashSet<u32>)> = s
            .trim()
            .split("\n")
            .map(|s| {
                let (s1, s2) = s.trim().split_once(',').unwrap();
                let e1: (u32, u32) = s1
                    .split_once('-')
                    .and_then(|(a, b)| Some((a.parse().unwrap(), b.parse().unwrap())))
                    .unwrap();

                let e2: (u32, u32) = s2
                    .split_once('-')
                    .and_then(|(a, b)| Some((a.parse().unwrap(), b.parse().unwrap())))
                    .unwrap();

                (
                    HashSet::from_iter(e1.0..=e1.1),
                    HashSet::from_iter(e2.0..=e2.1),
                )
            })
            .collect();
        Ok(Self { data })
    }
}

impl Elves {
    pub fn solve_part1(&self) -> u32 {
        self.data.iter().fold(0, |acc, (s1, s2)| {
            if s1.is_subset(s2) || s1.is_superset(s2) {
                acc + 1
            } else {
                acc
            }
        })
    }
    pub fn solve_part2(&self) -> u32 {
        self.data.iter().fold(0, |acc, (s1, s2)| {
            if let Some(_) = s1.intersection(s2).next() {
                acc + 1
            } else {
                acc
            }
        })
    }
}

#[tokio::main]
async fn main() {
    let input = lib::get_input(2022, 4).await;

    let now = Instant::now();
    let elves: Elves = input.parse().unwrap();
    println!("Parsing: {:?}", now.elapsed());

    let now = Instant::now();
    let answer = elves.solve_part1();
    println!("Timing 1: {:?}", now.elapsed());
    println!("Part 1: {}", answer);

    let now = Instant::now();
    let answer = elves.solve_part2();
    println!("Timing 1: {:?}", now.elapsed());
    println!("Part 2: {}", answer);
}

#[cfg(test)]
mod test {
    use std::collections::HashSet;

    #[test]
    fn superset() {
        let h1: HashSet<u32> = HashSet::from_iter(0..=5);
        let h2: HashSet<u32> = HashSet::from_iter(0..=10);
        assert_eq!(h1.len(), 6);
        assert_eq!(h2.len(), 11);
        assert_eq!(h1.is_superset(&h2), false);
        assert_eq!(h2.is_superset(&h1), true);
    }
}
