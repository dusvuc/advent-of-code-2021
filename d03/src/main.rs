use std::fs;

fn open_file(path_s: &str) -> (usize, Vec<u64>) {
    let data_str = match fs::read_to_string(path_s) {
        Err(why) => panic!("couldn't open {}: {}", path_s, why.to_string()),
        Ok(string) => string,
    };

    let mut data: Vec<u64> = vec![];

    let length = data_str.lines().nth(0).unwrap().len();

    for line in data_str.lines() {
        data.push(match u64::from_str_radix(line, 2) {
            Ok(x) => x,
            Err(_) => {
                panic!("Cannot parse!");
            }
        });
    }
    (length, data)
}

fn get_gamma_epsilon_rate(length: usize, data: &Vec<u64>) -> (u64, u64) {
    let mut gamma_rate: u64 = 0;
    let mut epsilon_rate: u64 = 0;
    for i in 0..length {
        let mut ones_cnt = 0;
        let mut zeroes_cnt = 0;
        for number in data.iter() {
            if number & (1 << i) == 0 {
                zeroes_cnt += 1;
            } else {
                ones_cnt += 1;
            }
        }
        if ones_cnt > zeroes_cnt {
            gamma_rate |= 1 << i;
        } else {
            epsilon_rate |= 1 << i;
        }
    }
    (gamma_rate, epsilon_rate)
}

fn get_number_of_values_at_position(position: usize, ones: bool, data: &Vec<u64>) -> u64 {
    data.iter().fold(0, |acc, x| {
        let has_one: bool = (x & (1 << position)) != 0;
        if (has_one && ones) || (!has_one && !ones) {
            return acc + 1;
        }
        acc
    })
}

fn get_filtered_vector(position: usize, ones: bool, data: Vec<u64>) -> Vec<u64> {
    let result = data
        .into_iter()
        .filter_map(|num| {
            let has_one: bool = (num & (1 << position)) != 0;
            if (has_one && ones) || (!has_one && !ones) {
                return Some(num);
            }
            return None;
        })
        .collect();
    result
}

fn get_rating(length: usize, mut data: Vec<u64>, oxy: bool) -> u64 {
    let mut position: usize = length;
    while data.len() > 1 {
        position = position - 1;
        let zeroes = get_number_of_values_at_position(position, false, &data);
        let ones = get_number_of_values_at_position(position, true, &data);
        if zeroes > ones {
            data = get_filtered_vector(position, !oxy, data);
        } else if ones > zeroes {
            data = get_filtered_vector(position, oxy, data);
        } else {
            data = get_filtered_vector(position, oxy, data);
        }
    }
    data[0]
}

fn main() {
    let (length, data) = open_file("input");
    let (gamma_rate, epsilon_rate) = get_gamma_epsilon_rate(length, &data);
    println!(
        "({}, {}, {})",
        gamma_rate,
        epsilon_rate,
        gamma_rate * epsilon_rate
    );
    let oxy = get_rating(length, data.clone(), true);
    let co2 = get_rating(length, data.clone(), false);
    println!("({}, {}, {})", oxy, co2, oxy * co2);
}
