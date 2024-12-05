const INPUT: &str = include_str!("../inputs/02");

pub fn main() {
    let mut safe_reports = 0;

    for line in INPUT.lines() {
        let nums = line
            .split_whitespace()
            .map(|x| x.parse().unwrap())
            .collect::<Vec<isize>>();

        'outer: for i in 0..nums.len() {
            let nums = nums[0..i]
                .iter()
                .chain(nums[i + 1..].iter())
                .collect::<Vec<_>>();

            let increasing = nums[1] > nums[0];

            for i in 1..nums.len() {
                if increasing && !(1..=3).contains(&(nums[i] - nums[i - 1])) {
                    continue 'outer;
                } else if !increasing && !(1..=3).contains(&(nums[i - 1] - nums[i])) {
                    continue 'outer;
                }
            }

            safe_reports += 1;
            break;
        }
    }

    println!("{safe_reports}");
}
