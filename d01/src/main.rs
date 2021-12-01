use std::fs;

fn open_file(path_s: &str) -> Vec<u32> {
    let data_str = match fs::read_to_string(path_s) {
        Err(why) => panic!("couldn't open {}: {}", path_s, why.to_string()),
        Ok(string) => string,
    };

    let mut data: Vec<u32> = vec![];

    for str in data_str.lines() {
        data.push(match str.trim().parse() {
            Ok(x) => x,
            Err(_) => continue,
        });
    }
    data
}

fn find_increases(v: &Vec<u32>) -> u32 {
    let mut res: u32 = 0;
    let mut previous_value = u32::MAX;
    for val in v.iter() {
        if *val > previous_value {
            res += 1;
        }
        previous_value = *val;
    }

    res
}

fn main() {
    let vec = open_file("input");
    let increases = find_increases(&vec);
    println!("Increases: {}.", increases)
}
