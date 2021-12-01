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

fn find_sliding(v: &Vec<u32>) -> u32 {
    let mut res: u32 = 0;

    let mut last_sum = v[0] + v[1] + v[2];
    let mut last_ptr: usize = 0;
    for i in 3..v.len() {
        let new_sum = last_sum + v[i] - v[last_ptr];
        if new_sum > last_sum {
            res += 1;
        }
        last_ptr += 1;
        last_sum = new_sum;
    }
    res
}

fn main() {
    let vec = open_file("input");
    let increases = find_increases(&vec);
    let sliding = find_sliding(&vec);
    println!("Increases: {}, sliding: {}.", increases, sliding);
}
