use std::collections::BTreeSet;
use std::iter::FromIterator;

fn main() {
    let lines = include_str!("input.txt");

    // Part 1
    let total_priority = lines
        .lines()
        .map(|line| {
            let (c1, c2) = line.split_at(line.len() / 2);
            let s1 = BTreeSet::from_iter(c1.chars());
            let s2 = BTreeSet::from_iter(c2.chars());
            let shared_item = s1.intersection(&s2).nth(0).unwrap().clone();
            match shared_item {
                'a'..='z' => 1 + shared_item as u32 - 'a' as u32,
                'A'..='Z' => 27 + shared_item as u32 - 'A' as u32,
                _ => panic!("invalid item: {}", shared_item),
            }
        })
        .sum::<u32>();
    println!("Sum of priorities is {}", total_priority,);
}
