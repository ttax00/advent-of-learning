use std::{collections::HashSet, str::FromStr, string::ParseError, time::Instant};

struct Elves {
    data: Vec<(HashSet<u32>, HashSet<u32>)>,
}

impl FromStr for Elves {
    type Err = ParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let data = s
            .trim()
            .split('\n')
            .map(|s| {
                let s = s.trim();
                let half1 = &s[0..s.len() / 2];
                let half2 = &s[s.len() / 2..];
                let get_hash = |s: &str| -> HashSet<u32> {
                    s.chars()
                        .map(|c| match c as u32 {
                            65..=90 => c as u32 - 38,
                            97..=122 => c as u32 - 96,
                            _ => unreachable!(),
                        })
                        .collect()
                };
                (get_hash(half1), get_hash(half2))
            })
            .collect();
        Ok(Self { data })
    }
}

impl Elves {
    pub fn solve_part1(&self) -> u32 {
        self.data
            .iter()
            .fold(0, |acc, (h1, h2)| acc + h1.intersection(h2).next().unwrap())
    }

    pub fn solve_part2(&self) -> u32 {
        let mut data = self.data.iter();
        let mut sum = 0;
        while let Some((f1, f2)) = data.next() {
            let (s1, s2) = data.next().unwrap();
            let (t1, t2) = data.next().unwrap();
            let first: HashSet<&u32> = f1.union(f2).collect();
            let second: HashSet<&u32> = s1.union(s2).collect();
            let third: HashSet<&u32> = t1.union(t2).collect();
            let inter1: HashSet<&&u32> = first.intersection(&second).collect();
            let inter2: HashSet<&&u32> = second.intersection(&third).collect();
            let fin = inter1.intersection(&inter2).next().unwrap();
            sum += **fin;
        }
        sum
    }
}

#[tokio::main]
async fn main() {
    let input = lib::get_input(2022, 3).await;

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
    use super::*;
    const SAMPLE: &str = r#"
	vJrwpWtwJgWrhcsFMMfFFhFp
	jqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL
	PmmdzqPrVvPwwTWBwg
	wMqvLMZHhHMvwLHjbvcjnnSBnvTQFn
	ttgJtRGJQctTZtZT
	CrZsJsPPZsGzwwsLwLmpwMDw"#;
    #[test]
    fn test_parse() {
        let _e: Elves = SAMPLE.parse().unwrap();
    }
}
