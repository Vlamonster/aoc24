use regex::Regex;

const INPUT: &str = include_str!("../inputs/03");

pub fn main() {
    let mut total = 0;
    let regex = Regex::new(r"mul\((\d{1,3}),(\d{1,3})\)").unwrap();
    for segment in INPUT.split("do()").collect::<Vec<_>>() {
        for m in regex.captures_iter(segment.split("don't()").next().unwrap()) {
            total += m[1].parse::<usize>().unwrap() * m[2].parse::<usize>().unwrap();
        }
    }
    println!("{total}");
}
