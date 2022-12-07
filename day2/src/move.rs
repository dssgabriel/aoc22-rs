use crate::outcome::Outcome;

#[derive(Debug, Clone, Copy)]
pub enum Move {
    Rock,
    Paper,
    Scissors,
}

impl Move {
    const ALL_MOVES: [Move; 3] = [Move::Rock, Move::Paper, Move::Scissors];

    /// Returns points depending on the chosen move
    pub fn points_per_move(self) -> u64 {
        match self {
            Move::Rock => 1,
            Move::Paper => 2,
            Move::Scissors => 3,
        }
    }

    /// Returns whether a move beats another
    fn beats(self, other: Move) -> bool {
        matches! {
            (self, other),
            (Self::Rock, Self::Scissors) | (Self::Scissors, Self::Paper) | (Self::Paper, Self::Rock)
        }
    }

    /// Outcome of two moves
    pub fn outcome(self, other: Move) -> Outcome {
        if self.beats(other) {
            Outcome::Win
        } else if other.beats(self) {
            Outcome::Loss
        } else {
            Outcome::Draw
        }
    }

    /// Returns the winning move against `self`
    pub fn winning_move(self) -> Self {
        Self::ALL_MOVES
            .iter()
            .copied()
            .find(|m| m.beats(self))
            .expect("at least one move beats another")
    }

    /// Returns the losing move against `self`
    pub fn losing_move(self) -> Self {
        Self::ALL_MOVES
            .iter()
            .copied()
            .find(|&m| self.beats(m))
            .expect("at least one move beats another")
    }

    /// Returns the drawing move, i.e. `self`
    pub fn drawing_move(self) -> Self {
        self
    }
}

impl TryFrom<char> for Move {
    type Error = color_eyre::Report;

    fn try_from(value: char) -> Result<Self, Self::Error> {
        match value {
            'A' => Ok(Move::Rock),
            'B' => Ok(Move::Paper),
            'C' => Ok(Move::Scissors),
            _ => Err(color_eyre::eyre::eyre!("not a valid move: {value:?}")),
        }
    }
}
