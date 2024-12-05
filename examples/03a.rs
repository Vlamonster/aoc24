use regex::Regex;

const INPUT: &str = include_str!("../inputs/03");

pub fn main() {
    let mut total = 0;
    let regex = Regex::new(r"mul\((\d{1,3}),(\d{1,3})\)").unwrap();
    for m in regex.captures_iter(INPUT) {
        total += m[1].parse::<usize>().unwrap() * m[2].parse::<usize>().unwrap();
    }
    println!("{total}");
}
