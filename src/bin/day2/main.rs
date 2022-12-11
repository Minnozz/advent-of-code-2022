// A, X, 0: Rock (1 pt)
// B, Y, 1: Paper (2 pt)
// C, Z, 2: Scissors (3 pt)

// X, 0: Loss (0 pt)
// Y, 1: Draw (3 pt)
// Z, 2: Win (6 pt)

fn main() {
    let lines = include_str!("input.txt").lines();

    // Part 1
    let total_score = lines
        .map(|line| {
            let chars = line.chars().collect::<Vec<_>>();
            let their_move = chars[0] as i32 - 'A' as i32;
            let our_move = chars[2] as i32 - 'X' as i32;
            let outcome = match (their_move - our_move + 3) % 3 {
                0 => 3, // Draw
                1 => 0, // Loss
                2 => 6, // Win
                _ => panic!("invalid line: {}", line),
            };
            our_move + 1 + outcome
        })
        .sum::<i32>();
    println!("Total score: {}", total_score);
}
