use std::{
    collections::{HashMap, HashSet},
    ffi::OsStr,
    path::{Path, PathBuf},
};

#[tokio::main]
async fn main() {
    let input = lib::get_input(2022, 7).await;

    assert_eq!(
        part1(
            r"
$ cd /
$ ls
dir a
14848514 b.txt
8504156 c.dat
dir d
$ cd a
$ ls
dir e
29116 f
2557 g
62596 h.lst
$ cd e
$ ls
584 i
$ cd ..
$ cd ..
$ cd d
$ ls
4060174 j
8033020 d.log
5626152 d.ext
7214296 k
"
            .trim()
        ),
        95437
    );

    let answer = part1(&input);
    println!("Part 1: {answer}");
    let answer = part2(&input);
    println!("Part 2: {answer}");
}

#[derive(Debug)]
enum File {
    Dir(String),
    File(u32),
}

fn parse_file_tree(s: &str) -> HashMap<String, Vec<File>> {
    let mut path = PathBuf::from("/");
    let mut hm: HashMap<String, Vec<File>> = HashMap::default();
    for cmd in s.split("$ ") {
        let mut test = cmd
            .split("\n")
            .filter(|s| !s.is_empty())
            .collect::<Vec<_>>();
        if test.len() == 1 {
            let (_, relative_dir) = test[0].split_once(" ").unwrap();
            if relative_dir == ".." {
                path.pop();
            } else {
                path.push(relative_dir);
            }
        } else if test.len() > 1 {
            assert_eq!(test.remove(0), "ls");
            let mut v = vec![];
            for file in test {
                let (t, name) = file.split_once(" ").unwrap();
                if let Ok(n) = t.parse::<u32>() {
                    v.push(File::File(n));
                } else {
                    v.push(File::Dir(path.join(name).to_string_lossy().to_string()));
                }
            }
            hm.insert(path.to_string_lossy().to_string(), v);
        }
    }
    return hm;
}

fn get_dir_size(dir: &str, hm: &HashMap<String, Vec<File>>) -> u32 {
    let content = hm.get(dir).expect(&format!("cannot find dir {}", dir));
    let mut size = 0;
    for f in content.into_iter() {
        size += match f {
            File::Dir(s) => get_dir_size(s, hm),
            File::File(n) => *n,
        }
    }
    size
}

fn part1(s: &str) -> u32 {
    let hm = parse_file_tree(s);
    let v =
        hm.keys()
            .map(|k| get_dir_size(k, &hm))
            .fold(0, |acc, n| if n <= &100000 { acc + n } else { acc });
    v
}

fn part2(s: &str) -> u32 {
    let hm = parse_file_tree(s);
    let mut sizes = hm.keys().map(|k| get_dir_size(k, &hm)).collect::<Vec<_>>();
    sizes.sort_by(|a, b| Ord::cmp(&b, &a));

    let system = sizes.get(0).unwrap();
    let install = 30_000_000;
    let total = 70_000_000;
    let need_free = system + install - total;
    sizes
        .iter()
        .fold(0, |acc, n| if n >= &need_free { *n } else { acc })
}
