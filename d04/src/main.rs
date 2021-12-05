use std::fs;

#[derive(Debug)]
struct Marker {
    markings: [[bool; 5]; 5],
}

impl Marker {
    fn mark(&mut self, i: usize, j: usize) {
        self.markings[i][j] = true;
    }

    fn check_row(&self, row: usize) -> bool {
        for j in 0..5 {
            if !self.markings[row][j] {
                return false;
            }
        }
        true
    }

    fn check_column(&self, column: usize) -> bool {
        for i in 0..5 {
            if !self.markings[i][column] {
                return false;
            }
        }
        true
    }
}

#[derive(Debug)]
struct Board {
    numbers: [[usize; 5]; 5],
    marker: Marker,
    won: bool,
}

impl Board {
    fn update(&mut self, number: usize) -> (bool, usize, usize) {
        match self.find_number(number) {
            Some((i, j)) => {
                self.marker.mark(i, j);
                return (true, i, j);
            }
            None => {
                return (false, 0, 0);
            }
        }
    }

    fn is_bingo(&self, i: usize, j: usize) -> bool {
        return self.marker.check_row(i) || self.marker.check_column(j);
    }

    fn get_unmarked_sum(&self) -> usize {
        let mut result: usize = 0;
        for i in 0..5 {
            for j in 0..5 {
                if !self.marker.markings[i][j] {
                    result += self.numbers[i][j];
                }
            }
        }
        result
    }

    fn find_number(&self, number: usize) -> Option<(usize, usize)> {
        for i in 0..5 {
            for j in 0..5 {
                if self.numbers[i][j] == number {
                    return Some((i, j));
                }
            }
        }
        return None;
    }

    fn from_values(values: &Vec<usize>) -> Board {
        let mut board = Board {
            numbers: [[0; 5]; 5],
            marker: Marker {
                markings: [[false; 5]; 5],
            },
            won: false,
        };

        for i in 0..5 {
            for j in 0..5 {
                board.numbers[i][j] = values[i * 5 + j];
            }
        }

        board
    }

    fn from_board_positions(mut numbers: Vec<usize>) -> Vec<Board> {
        let mut result: Vec<Board> = Vec::new();
        while numbers.len() > 0 {
            let new_vector = numbers.split_off(25);
            let board = Board::from_values(&numbers);
            numbers = new_vector;
            result.push(board);
        }
        result
    }
}

fn open_file(path_s: &str) -> (Vec<usize>, Vec<usize>) {
    let data_str = match fs::read_to_string(path_s) {
        Err(why) => panic!("couldn't open {}: {}", path_s, why.to_string()),
        Ok(string) => string,
    };

    let mut lines = data_str.lines();
    let first_line = lines.next().unwrap();
    let numbers: Vec<usize> = first_line
        .split(",")
        .map(|str_num| str_num.parse::<usize>().unwrap())
        .collect();

    // from second line to the end
    let boards: Vec<usize> = lines
        .filter(|line| !line.is_empty())
        .map(|line| {
            line.split_whitespace()
                .map(|str_num| str_num.parse::<usize>().unwrap())
        })
        .flatten()
        .collect();

    (numbers, boards)
}

fn calculate_bingo_score(numbers: Vec<usize>, mut boards: Vec<Board>) {
    for number in numbers {
        for board in boards.iter_mut() {
            let (changed, i, j) = board.update(number);
            if changed && !board.won && board.is_bingo(i, j) {
                board.won = true;
                println!(
                    "Board winning state is {} for number {}!",
                    board.get_unmarked_sum() * number,
                    number
                );
            }
        }
    }
}

fn main() {
    let (numbers, board_positions) = open_file("/home/dusan/advent-of-code-2021/d04/input");
    let boards = Board::from_board_positions(board_positions);
    calculate_bingo_score(numbers, boards);
}
