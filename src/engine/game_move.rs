use super::square::Square;
use std::fmt;

pub struct GameMove {
    pub from: Square,
    pub to: Square,
}

impl GameMove {
    pub fn manhattan_distance(&self) -> usize {
        let file_distance = (self.from.file() as i8 - self.to.file() as i8).abs();
        let rank_distance = (self.from.rank() as i8 - self.to.rank() as i8).abs();
        (file_distance + rank_distance) as usize
    }
}

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
