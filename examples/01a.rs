const INPUT: &str = include_str!("../inputs/01");

pub fn main() {
    let mut list_1 = Vec::<usize>::new();
    let mut list_2 = Vec::<usize>::new();

    for line in INPUT.lines() {
        let mut segments = line.split_whitespace();
        list_1.push(segments.next().unwrap().parse().unwrap());
        list_2.push(segments.next().unwrap().parse().unwrap());
    }

    list_1.sort();
    list_2.sort();

    let result = list_1
        .into_iter()
        .zip(list_2.into_iter())
        .map(|(x, y)| x.abs_diff(y))
        .sum::<usize>();

    println!("{result}");
}
