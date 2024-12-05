use std::collections::HashMap;

const INPUT: &str = include_str!("../inputs/01");

pub fn main() {
    let mut list = Vec::<usize>::new();
    let mut map = HashMap::<usize, usize>::new();

    for line in INPUT.lines() {
        let mut segments = line.split_whitespace();
        list.push(segments.next().unwrap().parse().unwrap());
        *map.entry(segments.next().unwrap().parse().unwrap())
            .or_default() += 1;
    }

    let result = list
        .into_iter()
        .map(|x| x * map.get(&x).unwrap_or(&0))
        .sum::<usize>();

    println!("{result}");
}
