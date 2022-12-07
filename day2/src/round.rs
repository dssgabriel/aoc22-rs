use crate::outcome::Outcome;
use crate::r#move::Move;

use std::str::FromStr;

#[derive(Debug, Clone, Copy)]
pub struct Round {
    rhs: Move,
    lhs: Move,
}

impl Round {
    fn outcome(self) -> Outcome {
        self.rhs.outcome(self.lhs)
    }

    pub fn rhs_score(self) -> u64 {
        self.rhs.points_per_move() + self.outcome().points_per_outcome()
    }
}

impl FromStr for Round {
    type Err = color_eyre::Report;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut chars = s.chars();
        let (Some(lhs), Some(' '), Some(rhs), None) = (chars.next(), chars.next(), chars.next(), chars.next()) else {
            return Err(color_eyre::eyre::eyre!("expected `<lhs> <rhs>\n`, got {s:?}"));
        };
        let lhs = Move::try_from(lhs)?;
        let outcome = Outcome::try_from(rhs)?;
        let rhs = outcome.matching_move(lhs);

        Ok(Self { lhs, rhs })
    }
}
