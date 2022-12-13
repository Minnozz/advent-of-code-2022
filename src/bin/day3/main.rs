use std::collections::BTreeSet;
use std::iter::FromIterator;

use itertools::Itertools;

fn priority(item: char) -> u32 {
    match item {
        'a'..='z' => 1 + item as u32 - 'a' as u32,
        'A'..='Z' => 27 + item as u32 - 'A' as u32,
        _ => panic!("invalid item: {}", item),
    }
}

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
            return priority(shared_item);
        })
        .sum::<u32>();
    println!("Sum of priorities is {}", total_priority,);

    // Part 2
    let total_badge_priority = lines
        .lines()
        .map(|line| BTreeSet::from_iter(line.chars()))
        .chunks(3)
        .into_iter()
        .map(|chunk| {
            let badge_item = chunk
                .reduce(|a, b| a.intersection(&b).cloned().collect())
                .unwrap()
                .iter()
                .nth(0)
                .unwrap()
                .clone();
            priority(badge_item)
        })
        .sum::<u32>();
    println!("Sum of badge priorities is {}", total_badge_priority);
}
