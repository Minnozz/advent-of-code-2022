// A, X, 0: Rock (1 pt)
// B, Y, 1: Paper (2 pt)
// C, Z, 2: Scissors (3 pt)

// X, 0: Loss (0 pt)
// Y, 1: Draw (3 pt)
// Z, 2: Win (6 pt)

fn main() {
    let lines = include_str!("input.txt");

    // Part 1
    let total_score = lines
        .lines()
        .map(|line| {
            let chars = line.chars().collect::<Vec<_>>();
            let their_move = chars[0] as i32 - 'A' as i32;
            let our_move = chars[2] as i32 - 'X' as i32;
            let outcome = (our_move - their_move + 4) % 3;
            return (our_move + 1) + (outcome * 3);
        })
        .sum::<i32>();
    println!("Total score by playing moves: {}", total_score);

    // Part 2
    let total_score = lines
        .lines()
        .map(|line| {
            let chars = line.chars().collect::<Vec<_>>();
            let their_move = chars[0] as i32 - 'A' as i32;
            let outcome = chars[2] as i32 - 'X' as i32;
            let our_move = (outcome + their_move + 2) % 3;
            assert!(outcome == (our_move - their_move + 4) % 3);
            return (our_move + 1) + (outcome * 3);
        })
        .sum::<i32>();
    println!("Total score by playing to outcome: {}", total_score);
}
