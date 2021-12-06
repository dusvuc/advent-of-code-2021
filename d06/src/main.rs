use std::fs;

fn open_file(path_s: &str) -> Vec<Lanternfish> {
    let data_str = match fs::read_to_string(path_s) {
        Err(why) => panic!("couldn't open {}: {}", path_s, why.to_string()),
        Ok(string) => string,
    };

    let mut lines = data_str.lines();
    let first_line = lines.next().unwrap();
    let res: Vec<Lanternfish> = first_line
        .split(",")
        .map(|str_age| Lanternfish::new_lanternfish(str_age.parse::<usize>().unwrap()))
        .collect();
    res
}

#[derive(Debug, Clone)]
struct Lanternfish {
    age: usize,
}

impl Lanternfish {
    fn fresh_lanternfish() -> Lanternfish {
        Lanternfish { age: 8 }
    }

    fn new_lanternfish(age: usize) -> Lanternfish {
        Lanternfish { age }
    }

    fn process(&mut self) -> Option<Lanternfish> {
        if self.age == 0 {
            self.age = 6;
            return Some(Lanternfish::fresh_lanternfish());
        }
        self.age -= 1;
        None
    }
}

fn lanternfish_after_days(mut lanternfish_fleet: Vec<Lanternfish>, days: usize) -> usize {
    for _ in 0..days {
        let mut new_lanternfish_fleet: Vec<Lanternfish> = Vec::new();
        for lanternfish in lanternfish_fleet.iter_mut() {
            match lanternfish.process() {
                Some(new_lanternfish) => {
                    new_lanternfish_fleet.push(new_lanternfish);
                }
                None => {}
            }
        }
        lanternfish_fleet.extend(new_lanternfish_fleet);
    }

    lanternfish_fleet.len()
}

fn main() {
    let start_lanternfish = open_file("input");
    let res1 = lanternfish_after_days(start_lanternfish.clone(), 80);
    println!("{}", res1);
    //let res2 = lanternfish_after_days(start_lanternfish.clone(), 256);
    //println!("{}", res2);
}
