use std::collections::HashSet;
use std::fs;

fn open_file(path_s: &str) -> (Vec<Vec<String>>, Vec<Vec<String>>) {
    let data_str = match fs::read_to_string(path_s) {
        Err(why) => panic!("couldn't open {}: {}", path_s, why.to_string()),
        Ok(string) => string,
    };

    let mut first_half: Vec<Vec<String>> = Vec::new();
    let mut second_half: Vec<Vec<String>> = Vec::new();
    for line in data_str.lines() {
        let (left_str, right_str) = line.split_once(" | ").unwrap();
        let v1: Vec<String> = left_str.split_whitespace().map(|x| x.to_string()).collect();
        let v2: Vec<String> = right_str
            .split_whitespace()
            .map(|x| {
                let mut v = x.chars().collect::<Vec<char>>();
                v.sort();
                v.into_iter().collect()
            })
            .collect();
        first_half.push(v1);
        second_half.push(v2);
    }
    (first_half, second_half)
}

fn get_part_one(v2: &Vec<Vec<String>>) -> usize {
    let mut count: usize = 0;
    // corresponds to 1, 4, 7, 8
    let set: HashSet<usize> = HashSet::from_iter(vec![2, 3, 4, 7]);
    for vv in v2 {
        for v1 in vv {
            if set.contains(&v1.len()) {
                count += 1;
            }
        }
    }
    count
}

fn main() {
    let (v1, v2) = open_file("input");
    let r1 = get_part_one(&v2);
    let r2 = 0;
    println!("{}, {:?}", r1, r2);
}
