use super::square::Square;
use std::fmt;

pub struct GameMove {
    pub from: Square,
    pub to: Square,
}

impl GameMove {}

impl fmt::Debug for GameMove {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}{}",
            self.from.to_algebraic_notation(),
            self.to.to_algebraic_notation()
        )
    }
}
