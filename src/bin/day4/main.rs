use itertools::Itertools;

fn parse_range(range: &str) -> (u32, u32) {
    let (from, to) = range.split_once('-').unwrap();
    return (from.parse().unwrap(), to.parse().unwrap());
}

fn contains((range_from, range_to): &(u32, u32), (other_from, other_to): &(u32, u32)) -> bool {
    other_from >= range_from && other_to <= range_to
}

fn disjoint((a_from, a_to): &(u32, u32), (b_from, b_to): &(u32, u32)) -> bool {
    a_to < b_from || b_to < a_from
}

fn main() {
    let lines = include_str!("input.txt");

    let ranges = lines
        .lines()
        .map(|line| {
            let (a, b) = line.split_once(',').unwrap();
            return (parse_range(a), parse_range(b));
        })
        .collect_vec();

    // Part 1
    let num_contained = ranges
        .iter()
        .filter(|(a, b)| contains(a, b) || contains(b, a))
        .count();
    println!(
        "Number of pairs where one is contained in the other: {}",
        num_contained
    );

    // Part 2
    let num_overlap = ranges.iter().filter(|(a, b)| !disjoint(a, b)).count();
    println!("Number of pairs with overlap: {}", num_overlap);
}
