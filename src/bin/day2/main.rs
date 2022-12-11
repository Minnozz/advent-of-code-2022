#[derive(PartialEq)]
enum Move {
    Rock = 1,
    Paper = 2,
    Scissors = 3,
}

impl From<&str> for Move {
    fn from(str: &str) -> Self {
        match str {
            "A" | "X" => Self::Rock,
            "B" | "Y" => Self::Paper,
            "C" | "Z" => Self::Scissors,
            _ => panic!("invalid move: {}", str),
        }
    }
}
enum Outcome {
    Loss = 0,
    Draw = 3,
    Win = 6,
}

impl From<&str> for Outcome {
    fn from(str: &str) -> Self {
        match str {
            "X" => Self::Loss,
            "Y" => Self::Draw,
            "Z" => Self::Win,
            _ => panic!("invalid outcome: {}", str),
        }
    }
}

fn battle(their: &Move, our: &Move) -> Outcome {
    if their == our {
        return Outcome::Draw;
    }
    match (their, our) {
        (Move::Rock, Move::Scissors)
        | (Move::Paper, Move::Rock)
        | (Move::Scissors, Move::Paper) => Outcome::Loss,
        (Move::Rock, Move::Rock)
        | (Move::Paper, Move::Paper)
        | (Move::Scissors, Move::Scissors) => Outcome::Draw,
        (Move::Rock, Move::Paper)
        | (Move::Paper, Move::Scissors)
        | (Move::Scissors, Move::Rock) => Outcome::Win,
    }
}

fn main() {
    let lines = include_str!("input.txt").lines();

    // Part 1
    let total_score = lines
        .map(|line| {
            let (theirs, ours) = line.split_once(" ").unwrap();
            let their_move: Move = theirs.into();
            let our_move: Move = ours.into();
            let outcome = battle(&their_move, &our_move);
            our_move as u32 + outcome as u32
        })
        .sum::<u32>();
    println!("Total score: {}", total_score);
}
