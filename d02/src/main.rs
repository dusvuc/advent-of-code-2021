use std::fs;
use std::str::FromStr;

fn open_file(path_s: &str) -> Vec<String> {
    let data_str = match fs::read_to_string(path_s) {
        Err(why) => panic!("couldn't open {}: {}", path_s, why.to_string()),
        Ok(string) => string,
    };

    let res: Vec<String> = data_str.lines().map(str::to_string).collect();
    res
}

fn get_course(lines: &Vec<String>) -> (i32, i32) {
    let (mut x_coord, mut y_coord) = (0, 0);
    for line in lines.iter() {
        let split_line = line.split_whitespace().collect::<Vec<&str>>();
        let direction = split_line[0];
        let amount = i32::from_str(split_line[1]).unwrap();
        match direction {
            "forward" => {
                x_coord += amount;
            }
            "down" => {
                y_coord += amount;
            }
            "up" => {
                y_coord -= amount;
            }
            _ => {
                println!("Error!");
                return (-1, -1);
            }
        }
    }
    (x_coord, y_coord)
}

fn get_modified_course(lines: &Vec<String>) -> (i32, i32) {
    let (mut x_coord, mut y_coord, mut aim) = (0, 0, 0);
    for line in lines.iter() {
        let split_line = line.split_whitespace().collect::<Vec<&str>>();
        let direction = split_line[0];
        let amount = i32::from_str(split_line[1]).unwrap();
        match direction {
            "forward" => {
                x_coord += amount;
                y_coord += aim * amount;
            }
            "down" => {
                aim += amount;
            }
            "up" => {
                aim -= amount;
            }
            _ => {
                println!("Error!");
                return (-1, -1);
            }
        }
    }
    (x_coord, y_coord)
}

fn main() {
    let lines = open_file("input");
    let (x_val, y_val) = get_course(&lines);
    println!("Total distance is {}", x_val * y_val);
    let (x_val, y_val) = get_modified_course(&lines);
    println!("Total modified_distance is {}", x_val * y_val);
}
