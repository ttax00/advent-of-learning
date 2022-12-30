use std::collections::{HashSet, VecDeque};

#[tokio::main]
async fn main() {
    let input = lib::get_input(2022, 6).await;

    let answer = part1(&input);
    println!("Part 1: {answer}");
    let answer = part2(&input);
    println!("Part 2: {answer}");
}

fn pos_after_marker(s: &String, window: usize) -> usize {
    let mut v: VecDeque<&str> = VecDeque::new();
    for (n, c) in s.split("").filter(|s| !s.is_empty()).enumerate() {
        v.push_back(c);
        if v.len() > window {
            v.pop_front();
        }
        if v.len() == window {
            let h: HashSet<&&str> = v.iter().collect();
            if h.len() == window {
                return n + 1;
            }
        }
    }
    unreachable!()
}

fn part1(s: &String) -> usize {
    pos_after_marker(s, 4)
}

fn part2(s: &String) -> usize {
    pos_after_marker(s, 14)
}
