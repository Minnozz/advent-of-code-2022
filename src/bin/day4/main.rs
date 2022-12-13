fn parse_range(range: &str) -> (u32, u32) {
    let (from, to) = range.split_once('-').unwrap();
    return (from.parse().unwrap(), to.parse().unwrap());
}

fn contains((range_from, range_to): (u32, u32), (other_from, other_to): (u32, u32)) -> bool {
    other_from >= range_from && other_to <= range_to
}

fn main() {
    let lines = include_str!("input.txt");

    let num_contained = lines
        .lines()
        .filter(|line| {
            let (a, b) = line.split_once(',').unwrap();
            let a_range = parse_range(a);
            let b_range = parse_range(b);
            return contains(a_range, b_range) || contains(b_range, a_range);
        })
        .count();

    // Part 1
    println!(
        "Number of pairs where one is contained in the other: {}",
        num_contained
    );
}
