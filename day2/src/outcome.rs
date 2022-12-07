use crate::r#move::Move;

#[derive(Debug, Clone, Copy)]
pub enum Outcome {
    Win,
    Draw,
    Loss,
}

impl Outcome {
    pub fn points_per_outcome(self) -> u64 {
        match self {
            Outcome::Win => 6,
            Outcome::Draw => 3,
            Outcome::Loss => 0,
        }
    }

    pub fn matching_move(self, lhs: Move) -> Move {
        match self {
            Outcome::Win => lhs.winning_move(),
            Outcome::Draw => lhs.drawing_move(),
            Outcome::Loss => lhs.losing_move(),
        }
    }
}

impl TryFrom<char> for Outcome {
    type Error = color_eyre::Report;

    fn try_from(c: char) -> Result<Self, Self::Error> {
        match c {
            'X' => Ok(Outcome::Loss),
            'Y' => Ok(Outcome::Draw),
            'Z' => Ok(Outcome::Win),
            _ => Err(color_eyre::eyre::eyre!("not a valid outcome: {c:?}")),
        }
    }
}
