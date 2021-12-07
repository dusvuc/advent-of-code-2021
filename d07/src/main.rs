use std::fs;

fn open_file(path_s: &str) -> Vec<i64> {
    let data_str = match fs::read_to_string(path_s) {
        Err(why) => panic!("couldn't open {}: {}", path_s, why.to_string()),
        Ok(string) => string,
    };

    let mut lines = data_str.lines();
    let first_line = lines.next().unwrap();
    let res: Vec<i64> = first_line
        .split(",")
        .map(|str_age| str_age.parse::<i64>().unwrap())
        .collect();
    res
}

fn get_value_for_position(positions: &Vec<i64>, selected_position: i64) -> i64 {
    let mut sum: i64 = 0;
    for position in positions {
        sum += (selected_position - position).abs();
    }
    sum
}

fn minimum_fuel(positions: &Vec<i64>) -> i64 {
    let res = positions
        .iter()
        .map(|x| get_value_for_position(&positions, *x))
        .min()
        .unwrap();
    res
}

fn get_values(num_of_values: usize) -> Vec<u64> {
    let mut res: Vec<u64> = Vec::new();
    let mut sum: u64 = 0;
    for i in 0..num_of_values {
        sum += i as u64;
        res.push(sum);
    }
    res.into_iter().map(|x| x as u64).collect()
}

fn get_new_value_for_position(
    positions: &Vec<i64>,
    values: &Vec<u64>,
    selected_position: i64,
) -> u64 {
    let mut sum: u64 = 0;
    for position in positions {
        let i = (selected_position - position).abs() as usize;
        sum += values[i];
    }
    sum
}

fn minimum_new_fuel(positions: &Vec<i64>) -> u64 {
    let values = get_values((*positions.iter().max().unwrap() + 2) as usize);

    let res = positions
        .iter()
        .map(|x| get_new_value_for_position(&positions, &values, *x - 1))
        .min()
        .unwrap();
    res
}

fn main() {
    let positions = open_file("input");
    let res1 = minimum_fuel(&positions);
    let res2 = minimum_new_fuel(&positions);
    println!("{}, {}", res1, res2);
}
