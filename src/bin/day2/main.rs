const LOSS: u32 = 0;
const DRAW: u32 = 3;
const WIN: u32 = 6;

fn main() {
    let lines = include_str!("input.txt").lines();

    let total_score = lines
        .map(|line| {
            let (theirs, ours) = line.split_once(" ").unwrap();
            let move_score = match ours {
                "X" => 1,
                "Y" => 2,
                "Z" => 3,
                _ => panic!("Invalid move {} in line {}", ours, line),
            };
            let win_score = match (theirs, ours) {
                ("A", "Z") | ("B", "X") | ("C", "Y") => LOSS,
                ("A", "X") | ("B", "Y") | ("C", "Z") => DRAW,
                ("A", "Y") | ("B", "Z") | ("C", "X") => WIN,
                _ => panic!("Invalid line: {}", line),
            };
            move_score + win_score
        })
        .sum::<u32>();

    // Part 1
    println!("Total score: {}", total_score);
}
