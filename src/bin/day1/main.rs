use itertools::Itertools;

fn main() {
    let lines = include_str!("input.txt").lines().collect_vec();
    let max_total = lines
        .split(|line| line.is_empty())
        .map(|lines| {
            lines
                .iter()
                .filter_map(|s| s.parse::<u32>().ok())
                .sum::<u32>()
        })
        .max();
    println!("{:?}", max_total);
}
