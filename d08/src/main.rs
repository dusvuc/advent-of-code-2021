use std::collections::HashMap;
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
        let v1: Vec<String> = left_str
            .split_whitespace()
            .map(|x| {
                let mut v = x.chars().collect::<Vec<char>>();
                v.sort();
                v.into_iter().collect()
            })
            .collect();
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

fn find_only_of_size(keys: &Vec<String>, size: usize) -> Vec<String> {
    let mut res: Vec<String> = Vec::new();
    for key in keys {
        if key.len() == size {
            res.push(key.to_string());
        }
    }
    res
}

fn get_a(keys: &Vec<String>) -> char {
    let s3 = find_only_of_size(&keys, 3);
    let s2 = find_only_of_size(&keys, 2);
    assert_eq!(s3.len(), 1);
    assert_eq!(s2.len(), 1);
    for c in s3[0].chars() {
        if !s2[0].contains(c) {
            return c;
        }
    }
    panic!("No matched character for {} and {}", s3[0], s2[0]);
}

fn remove_from_keys(keys: &Vec<String>, to_remove: char) -> Vec<String> {
    let mut res: Vec<String> = Vec::new();
    let mut strings: HashSet<String> = HashSet::new();
    for code in keys {
        let s: String = if code.contains(to_remove) {
            code.chars().filter(|x| *x != to_remove).collect()
        } else {
            code.to_string()
        };
        if s != "" && !strings.contains(&s) {
            strings.insert(s.to_string());
            res.push(s);
        }
    }

    res
}

fn get_sums_of(keys: &Vec<String>) -> HashMap<char, usize> {
    let mut res: HashMap<char, usize> = HashMap::new();
    for key in keys {
        for char in key.chars() {
            *res.entry(char).or_insert(0) += 1;
        }
    }

    res
}

fn get_e_b_f(sums: &HashMap<char, usize>) -> (char, char, char) {
    let mut e: char = 'q';
    let mut b: char = 'q';
    let mut f: char = 'q';

    for (k, v) in sums {
        if *v == 4 {
            e = *k;
        }
        if *v == 6 {
            b = *k;
        }
        if *v == 9 {
            f = *k;
        }
    }

    (e, b, f)
}

fn get_c(keys: &Vec<String>) -> char {
    let c_v = find_only_of_size(&keys, 1);
    assert_eq!(c_v.len(), 1);
    c_v[0].chars().next().unwrap()
}

fn find_d_g(keys: &Vec<String>, possible_d: char, possible_g: char) -> (char, char) {
    let s4 = find_only_of_size(&keys, 4);
    assert_eq!(s4.len(), 1);
    for c in s4[0].chars() {
        if c == possible_d {
            return (possible_d, possible_g);
        }
        if c == possible_g {
            return (possible_g, possible_d);
        }
    }
    panic!("Didn't match d or g.");
}

fn get_mapping(keys: &Vec<String>) -> HashMap<char, char> {
    let original_keys = keys.clone();
    let a_m = get_a(&keys);
    let (e_m, b_m, f_m) = get_e_b_f(&get_sums_of(&keys));
    let keys = remove_from_keys(&keys, e_m);
    let keys = remove_from_keys(&keys, b_m);
    let keys = remove_from_keys(&keys, f_m);
    let c_m = get_c(&keys);
    let keys = remove_from_keys(&keys, c_m);
    let keys = remove_from_keys(&keys, a_m);

    let sums = get_sums_of(&keys);

    /*
    We have d and g left over, we know d is contained in the initial four-character string
    */
    let mut keys = sums.keys();
    let possible_d = keys.next().unwrap();
    let possible_g = keys.next().unwrap();

    let (d_m, g_m) = find_d_g(&original_keys, *possible_d, *possible_g);

    HashMap::from([
        (a_m, 'a'),
        (b_m, 'b'),
        (c_m, 'c'),
        (d_m, 'd'),
        (e_m, 'e'),
        (f_m, 'f'),
        (g_m, 'g'),
    ])
}

fn decode_values(v1: &Vec<String>, v2: &Vec<String>, value_map: &HashMap<String, usize>) -> usize {
    let mapping = get_mapping(&v1);
    let unmapped_vec: Vec<String> = v2
        .clone()
        .into_iter()
        .map(|x| {
            let mut v = x.chars().map(|c| mapping[&c]).collect::<Vec<char>>();
            v.sort();
            v.iter().collect::<String>()
        })
        .collect();

    let numbers: Vec<usize> = unmapped_vec.iter().map(|x| value_map[x]).collect();

    numbers.into_iter().fold(0, |acc, x| acc * 10 + x)
}

fn get_part_two(v1: &Vec<Vec<String>>, v2: &Vec<Vec<String>>) -> usize {
    let value_map: HashMap<String, usize> = HashMap::from([
        ("abcefg".to_string(), 0 as usize),
        ("cf".to_string(), 1 as usize),
        ("acdeg".to_string(), 2 as usize),
        ("acdfg".to_string(), 3 as usize),
        ("bcdf".to_string(), 4 as usize),
        ("abdfg".to_string(), 5 as usize),
        ("abdefg".to_string(), 6 as usize),
        ("acf".to_string(), 7 as usize),
        ("abcdefg".to_string(), 8 as usize),
        ("abcdfg".to_string(), 9 as usize),
    ]);
    let mut sum: usize = 0;
    for (vv1, vv2) in v1.iter().zip(v2) {
        let r = decode_values(vv1, vv2, &value_map);
        sum += r;
    }

    return sum;
}

fn main() {
    let (v1, v2) = open_file("input");
    let r1 = get_part_one(&v2);
    let r2 = get_part_two(&v1, &v2);
    println!("{}, {:?}", r1, r2);
}
