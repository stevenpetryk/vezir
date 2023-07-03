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

        match (file_char, rank_char) {
            (Some(file_char), Some(rank_char))
                if ('a'..='h').contains(file_char) && ('1'..='8').contains(rank_char) =>
            {
                let file_index = (*file_char as usize) - ('a' as usize);
                let rank_index = (*rank_char as usize) - ('1' as usize);

                Ok(Square {
                    index: rank_index * 8 + file_index,
                })
            }
            _ => return Err(SquareParseError),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::assert_matches::assert_matches;

    #[test]
    fn from_algebraic_notation() {
        assert_matches!(Square::from_algebraic_notation("a1").unwrap().index, 0);
        assert_matches!(Square::from_algebraic_notation("b1").unwrap().index, 1);
        assert_matches!(Square::from_algebraic_notation("h8").unwrap().index, 63);

        // Invalid
        assert_matches!(
            Square::from_algebraic_notation("z9"),
            Err(SquareParseError {})
        );
        assert_matches!(
            Square::from_algebraic_notation("asdf"),
            Err(SquareParseError {})
        );
    }
}
