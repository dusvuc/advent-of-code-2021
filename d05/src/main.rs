use std::cmp;
use std::collections::HashMap;
use std::fs;

#[derive(Hash, PartialEq, Eq, Debug, Clone)]
struct Position {
    x: usize,
    y: usize,
}

#[derive(Debug, Clone)]
struct Line {
    start: Position,
    end: Position,
}

impl Line {
    fn is_horizontal(&self) -> bool {
        self.start.y == self.end.y
    }

    fn is_vertical(&self) -> bool {
        self.start.x == self.end.x
    }

    fn is_diagonal(&self) -> bool {
        let x1: i64 = self.start.x.try_into().unwrap();
        let x2: i64 = self.end.x.try_into().unwrap();
        let y1: i64 = self.start.y.try_into().unwrap();
        let y2: i64 = self.end.y.try_into().unwrap();

        let xdiff: i64 = x2 - x1;
        let ydiff: i64 = y2 - y1;

        xdiff.abs() == ydiff.abs()
    }

    fn get_path_horizontal(&self) -> Vec<Position> {
        let min_horizonal = cmp::min(self.start.x, self.end.x);
        let max_horizontal = cmp::max(self.start.x, self.end.x);
        let range = min_horizonal..max_horizontal + 1;
        range
            .into_iter()
            .map(|x| Position { x, y: self.start.y })
            .collect()
    }

    fn get_path_vertical(&self) -> Vec<Position> {
        let min_vertical = cmp::min(self.start.y, self.end.y);
        let max_vertical = cmp::max(self.start.y, self.end.y);
        let range = min_vertical..max_vertical + 1;
        range
            .into_iter()
            .map(|y| Position { x: self.start.x, y })
            .collect()
    }

    fn get_range(first: usize, second: usize) -> Vec<usize> {
        let mut res: Vec<usize> = Vec::new();
        let max = first.max(second);
        let min = first.min(second);
        let range = min..max + 1;
        if first < second {
            res.extend(range);
        } else {
            res.extend(range.rev());
        }
        res
    }

    fn get_path_diagonal(&self) -> Vec<Position> {
        let x_range: Vec<usize> = Line::get_range(self.start.x, self.end.x);
        let y_range: Vec<usize> = Line::get_range(self.start.y, self.end.y);
        let res = x_range
            .into_iter()
            .zip(y_range)
            .map(|(x, y)| Position { x, y })
            .collect();
        res
    }

    fn get_path(&self) -> Vec<Position> {
        if self.is_horizontal() {
            self.get_path_horizontal()
        } else if self.is_vertical() {
            self.get_path_vertical()
        } else if self.is_diagonal() {
            self.get_path_diagonal()
        } else {
            panic!("asdasd");
        }
    }
}

fn position_from_string(p: &str) -> Position {
    let (v1, v2) = p.split_once(",").unwrap();

    Position {
        x: v1.parse().unwrap(),
        y: v2.parse().unwrap(),
    }
}

fn line_from_string(line: &str) -> Line {
    let (left_pos, right_pos) = line.split_once(" -> ").unwrap();
    let (left_pos, right_pos) = (
        position_from_string(left_pos),
        position_from_string(right_pos),
    );
    Line {
        start: left_pos,
        end: right_pos,
    }
}

fn lines_from_input(path_s: &str) -> Vec<Line> {
    let data_str = match fs::read_to_string(path_s) {
        Err(why) => panic!("couldn't open {}: {}", path_s, why.to_string()),
        Ok(string) => string,
    };

    let lines = data_str
        .lines()
        .map(|line| line_from_string(line))
        .collect();
    lines
}

fn get_intersections(lines: Vec<Line>) -> usize {
    let mut hits: HashMap<(usize, usize), usize> = HashMap::new();
    let lines: Vec<Line> = lines
        .into_iter()
        .filter(|line| line.is_horizontal() || line.is_vertical())
        .collect();
    for line in lines {
        for new_position in line.get_path() {
            let mut value: usize = 0;
            if hits.contains_key(&(new_position.x, new_position.y)) {
                value = *hits.get(&(new_position.x, new_position.y)).unwrap();
            }
            hits.insert((new_position.x, new_position.y), value + 1);
        }
    }
    hits.values().filter(|&hit| hit > &1).count()
}

fn get_diagonal_intersections(lines: Vec<Line>) -> usize {
    let mut hits: HashMap<(usize, usize), usize> = HashMap::new();
    let lines: Vec<Line> = lines
        .into_iter()
        .filter(|line| line.is_horizontal() || line.is_vertical() || line.is_diagonal())
        .collect();
    for line in lines {
        for new_position in line.get_path() {
            let mut value: usize = 0;
            if hits.contains_key(&(new_position.x, new_position.y)) {
                value = *hits.get(&(new_position.x, new_position.y)).unwrap();
            }
            hits.insert((new_position.x, new_position.y), value + 1);
        }
    }
    hits.values().filter(|&hit| hit > &1).count()
}

fn main() {
    let lines = lines_from_input("input");
    println!("{}", get_intersections(lines.clone()));
    println!("{}", get_diagonal_intersections(lines.clone()));
}
