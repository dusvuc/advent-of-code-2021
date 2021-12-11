use std::collections::{HashSet, VecDeque};
use std::fs;

fn open_file(path_s: &str) -> Vec<Vec<usize>> {
    let data_str = match fs::read_to_string(path_s) {
        Err(why) => panic!("couldn't open {}: {}", path_s, why.to_string()),
        Ok(string) => string,
    };

    data_str
        .lines()
        .map(|line| {
            line.chars()
                .map(|c| c.to_digit(10).unwrap() as usize)
                .collect::<Vec<usize>>()
        })
        .collect()
}

fn find_first(data: &Vec<Vec<usize>>) -> usize {
    let (m, n) = (data.len(), data[0].len());

    let is_legal = |i: i64, j: i64| i >= 0 && i < m as i64 && j >= 0 && j < n as i64;
    let val_of = |i: i64, j: i64| {
        if is_legal(i as i64, j as i64) {
            data[i as usize][j as usize]
        } else {
            usize::MAX
        }
    };
    let is_min = |val, i, j| {
        val < val_of(i - 1, j)
            && val < val_of(i + 1, j)
            && val < val_of(i, j - 1)
            && val < val_of(i, j + 1)
    };

    let mut sum: usize = 0;

    for i in 0..m {
        for j in 0..n {
            if is_min(data[i][j], i as i64, j as i64) {
                sum += 1 + data[i][j];
            }
        }
    }

    sum
}

type Position = (usize, usize);
type Basin = Vec<Position>;

fn is_legal(i: i64, j: i64, m: usize, n: usize) -> bool {
    i >= 0 && i < m as i64 && j >= 0 && j < n as i64
}

fn insert_if_ok(
    visited: &mut HashSet<Position>,
    to_visit: &mut VecDeque<Position>,
    i: i64,
    j: i64,
    m: usize,
    n: usize,
) {
    if is_legal(i, j, m, n) && !visited.contains(&(i as usize, j as usize)) {
        visited.insert((i as usize, j as usize));
        to_visit.push_back((i as usize, j as usize));
    }
}

fn form_basin(
    data: &Vec<Vec<usize>>,
    visited: &mut HashSet<Position>,
    i: usize,
    j: usize,
) -> Basin {
    let mut basin: Basin = Vec::new();
    let mut to_visit: VecDeque<Position> = VecDeque::new();

    let (m, n) = (data.len(), data[0].len());

    to_visit.push_back((i, j));
    visited.insert((i, j));
    while !to_visit.is_empty() {
        let (i, j) = to_visit.pop_front().unwrap();
        basin.push((i, j));

        insert_if_ok(visited, &mut to_visit, i as i64 - 1, j as i64, m, n);
        insert_if_ok(visited, &mut to_visit, i as i64 + 1, j as i64, m, n);
        insert_if_ok(visited, &mut to_visit, i as i64, j as i64 - 1, m, n);
        insert_if_ok(visited, &mut to_visit, i as i64, j as i64 + 1, m, n);
    }

    basin
}

fn find_second(data: &Vec<Vec<usize>>) -> usize {
    let mut visited: HashSet<Position> = HashSet::new();
    let mut basins: Vec<Basin> = Vec::new();

    let (m, n) = (data.len(), data[0].len());

    for i in 0..m {
        for j in 0..n {
            if data[i][j] == 9 {
                visited.insert((i, j));
            }
        }
    }

    for i in 0..m {
        for j in 0..n {
            if !visited.contains(&(i, j)) {
                let basin = form_basin(&data, &mut visited, i, j);
                basins.push(basin);
            }
        }
    }

    let mut basin_sizes: Vec<usize> = basins.iter().map(|basin| basin.len()).collect();
    basin_sizes.sort_unstable();
    basin_sizes.reverse();

    basin_sizes[0] * basin_sizes[1] * basin_sizes[2]
}

fn main() {
    let input = open_file("input");
    let r1 = find_first(&input);
    let r2 = find_second(&input);
    println!("{}, {}", r1, r2);
}
