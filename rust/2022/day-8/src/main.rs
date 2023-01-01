#[tokio::main]
async fn main() {
    let input = lib::get_input(2022, 8).await;
    let test = r"
30373
25512
65332
33549
35390
		";
    // assert_eq!(part1(test), 21);
    assert_eq!(part2(test), 8);
    let v = parse_vec(test);
    assert_eq!(scenic_score(1, 2, &v), 4);
    assert_eq!(scenic_score(3, 2, &v), 8);

    let answer = part1(&input);
    println!("Part 1: {answer}");
    let answer = part2(&input);
    println!("Part 2: {answer}");
    assert_eq!(answer, 535680);
}

fn parse_vec(s: &str) -> Vec<Vec<u8>> {
    s.trim()
        .split("\n")
        .filter(|x| !x.is_empty())
        .map(|s| {
            s.trim()
                .split("")
                .filter(|x| !x.is_empty())
                .map(|s| s.parse::<u8>().unwrap())
                .collect()
        })
        .collect()
}

fn check_visibility(x: usize, y: usize, v: &Vec<Vec<u8>>) -> bool {
    if ns(x, y, v, (0..x).rev().collect(), false) == x
        || ns(x, y, v, (x + 1..v.len()).collect(), false) == v.len() - x - 1
        || we(x, y, v, (0..y).rev().collect(), false) == y
        || we(x, y, v, (y + 1..v[0].len()).collect(), false) == v[0].len() - y - 1
    {
        true
    } else {
        false
    }
}

fn ns(x: usize, y: usize, v: &Vec<Vec<u8>>, r: Vec<usize>, count_edge: bool) -> usize {
    let mut a = 0;
    for i in r {
        if v[i][y] < v[x][y] {
            a += 1;
        } else {
            if count_edge {
                return a + 1;
            } else {
                return a;
            }
        }
    }
    a
}

fn we(x: usize, y: usize, v: &Vec<Vec<u8>>, r: Vec<usize>, count_edge: bool) -> usize {
    let mut a = 0;
    for i in r {
        if v[x][i] < v[x][y] {
            a += 1;
        } else {
            if count_edge {
                return a + 1;
            } else {
                return a;
            }
        }
    }
    a
}

fn part1(s: &str) -> usize {
    let v = parse_vec(s);
    let mut visible = v.len() * v[0].len() - (v.len() - 2) * (v[0].len() - 2);
    for x in 1..v.len() - 1 {
        for y in 1..v[x].len() - 1 {
            // check north, south, east, west.
            if check_visibility(x, y, &v) {
                visible += 1;
            }
        }
    }
    visible
}

fn scenic_score(x: usize, y: usize, v: &Vec<Vec<u8>>) -> usize {
    let up = ns(x, y, v, (0..x).rev().collect(), true);
    let down = ns(x, y, v, (x + 1..v.len()).collect(), true);
    let left = we(x, y, v, (0..y).rev().collect(), true);
    let right = we(x, y, v, (y + 1..v[0].len()).collect(), true);
    up * down * left * right
}

fn part2(s: &str) -> usize {
    let mut v: Vec<usize> = vec![];
    let map = parse_vec(s);
    for x in 0..map.len() {
        for y in 0..map[x].len() {
            // check north, south, east, west.
            v.push(scenic_score(x, y, &map));
        }
    }
    v.sort();
    *v.last().unwrap()
}
