use std::collections::HashSet;

#[tokio::main]
async fn main() {
    let input = lib::get_input(2022, 9).await;

    let answer = part1(&input);
    println!("Part 1: {answer}");
    let answer = part2(&input);
    println!("Part 2: {answer}");
}

fn part1(s: &str) -> usize {
    let mut h = (0, 0);
    let mut t = (0, 0);
    let mut visited: HashSet<String> = HashSet::from([id(t)]);
    for line in s.trim().split("\n") {
        let (dir, n) = line.split_once(" ").unwrap();
        for _ in 0..n.parse::<usize>().unwrap() {
            match dir {
                "U" => h.0 += 1,
                "D" => h.0 -= 1,
                "L" => h.1 -= 1,
                "R" => h.1 += 1,
                _ => unreachable!(),
            }
            follow(&h, &mut t);
            visited.insert(id(t));
        }
    }
    visited.len()
}
fn id(n: (i32, i32)) -> String {
    format!("{}-{}", n.0, n.1)
}
fn follow(h: &(i32, i32), t: &mut (i32, i32)) {
    let dist = (((h.0 - t.0).pow(2) + (h.1 - t.1).pow(2)) as f32).sqrt();
    if dist > 2.0f32.sqrt() {
        if h.0 > t.0 {
            t.0 += 1;
        }
        if h.0 < t.0 {
            t.0 -= 1;
        }
        if h.1 > t.1 {
            t.1 += 1;
        }
        if h.1 < t.1 {
            t.1 -= 1;
        }
    }
}

fn part2(s: &str) -> usize {
    let mut h = (0, 0);
    let mut v = vec![
        (0, 0),
        (0, 0),
        (0, 0),
        (0, 0),
        (0, 0),
        (0, 0),
        (0, 0),
        (0, 0),
        (0, 0),
    ];
    let mut t = (0, 0);
    let mut visited: HashSet<String> = HashSet::from([id(h)]);
    for line in s.trim().split("\n") {
        let (dir, n) = line.split_once(" ").unwrap();
        for _ in 0..n.parse::<usize>().unwrap() {
            match dir {
                "U" => h.0 += 1,
                "D" => h.0 -= 1,
                "L" => h.1 -= 1,
                "R" => h.1 += 1,
                _ => unreachable!(),
            }
            for (i, x) in v.iter_mut().enumerate() {
                if i == 0 {
                    follow(&h, x);
                    t = x.clone();
                } else {
                    follow(&t, x);
                    t = x.clone();
                }
            }
            visited.insert(id(*v.get(8).unwrap()));
        }
    }

    visited.len()
}
