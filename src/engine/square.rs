#[derive(Debug)]
pub struct Square {
    pub index: usize,
}

#[derive(Debug, Clone)]
pub struct SquareParseError;

impl Square {
    pub fn from_algebraic_notation(notation: &str) -> Result<Square, SquareParseError> {
        let chars: Vec<char> = notation.chars().collect();
        let file_char = chars.get(0);
        let rank_char = chars.get(1);

        let (file, rank) = match (file_char, rank_char) {
            (Some(file_char), Some(rank_char)) => (
                (*file_char as usize - 'a' as usize),
                (*rank_char as usize) - ('1' as usize),
            ),
            _ => return Err(SquareParseError),
        };

        Ok(Square {
            index: rank * 8 + file,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::assert_matches::assert_matches;

    #[test]
    fn from_algabraic_notation() {
        assert_matches!(Square::from_algebraic_notation("a1").unwrap().index, 0);
        assert_matches!(Square::from_algebraic_notation("b1").unwrap().index, 1);
        assert_matches!(Square::from_algebraic_notation("h8").unwrap().index, 63);
    }
}
