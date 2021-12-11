use std::collections::HashMap;
use std::fs;

fn open_file(path_s: &str) -> Vec<String> {
    let data_str = match fs::read_to_string(path_s) {
        Err(why) => panic!("couldn't open {}: {}", path_s, why.to_string()),
        Ok(string) => string,
    };

    data_str.lines().map(|line| line.to_string()).collect()
}

fn get_line_score(score: &str) -> usize {
    let mut stack: Vec<char> = Vec::new();
    let symbols: HashMap<char, char> =
        HashMap::from([('[', ']'), ('(', ')'), ('{', '}'), ('<', '>')]);
    let symbols_r: HashMap<char, char> =
        HashMap::from([(']', '['), (')', '('), ('}', '{'), ('>', '<')]);
    let symbols_score: HashMap<char, usize> =
        HashMap::from([(')', 3), (']', 57), ('}', 1197), ('>', 25137)]);
    for c in score.chars() {
        if symbols.contains_key(&c) {
            stack.push(c);
        } else {
            if stack.is_empty() || symbols_r[&c] != stack[stack.len() - 1] {
                return symbols_score[&c];
            }
            stack.pop();
        }
    }
    0
}

fn fill_line_score(score: &str) -> usize {
    let mut stack: Vec<char> = Vec::new();
    let symbols: HashMap<char, char> =
        HashMap::from([('[', ']'), ('(', ')'), ('{', '}'), ('<', '>')]);
    let symbols_r: HashMap<char, char> =
        HashMap::from([(']', '['), (')', '('), ('}', '{'), ('>', '<')]);
    let symbols_score: HashMap<char, usize> =
        HashMap::from([(')', 1), (']', 2), ('}', 3), ('>', 4)]);
    for c in score.chars() {
        if symbols.contains_key(&c) {
            stack.push(c);
        } else {
            if stack.is_empty() || symbols_r[&c] != stack[stack.len() - 1] {
                continue;
            }
            stack.pop();
        }
    }

    stack.reverse();
    stack
        .iter()
        .map(|x| symbols[x])
        .fold(0, |acc, x| acc * 5 + symbols_score[&x])
}

fn get_first_result(input: &Vec<String>) -> usize {
    input.iter().map(|s| get_line_score(s)).sum()
}

fn get_second_result(input: &Vec<String>) -> usize {
    let input: Vec<String> = input
        .iter()
        .filter(|s| get_line_score(s) == 0)
        .map(|x| x.to_string())
        .collect();
    let mut res: Vec<usize> = input.iter().map(|s| fill_line_score(s)).collect();
    res.sort();
    res[res.len() / 2]
}

fn main() {
    let input = open_file("input");
    let r1 = get_first_result(&input);
    let r2 = get_second_result(&input);
    println!("{}, {}", r1, r2);
}
