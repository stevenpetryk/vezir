#[derive(Debug)]
pub struct Square {
    pub index: usize,
}

#[derive(Debug, Clone)]
pub struct SquareParseError;

impl Square {
    pub fn file(&self) -> usize {
        self.index % 8
    }

    pub fn rank(&self) -> usize {
        7 - self.index / 8
    }

    pub fn up(&self) -> Square {
        Square {
            index: self.index - 8,
        }
    }

    pub fn down(&self) -> Square {
        Square {
            index: self.index + 8,
        }
    }

    pub fn left(&self) -> Square {
        Square {
            index: self.index - 1,
        }
    }

    pub fn right(&self) -> Square {
        Square {
            index: self.index + 1,
        }
    }

    pub fn offset(&self, offset: &i8) -> Option<Square> {
        let new_index = self.index as i8 + offset;

        match new_index {
            0..=63 => Some(Square {
                index: new_index as usize,
            }),
            _ => None,
        }
    }

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
                    index: (7 - rank_index) * 8 + file_index,
                })
            }
            _ => return Err(SquareParseError),
        }
    }

    pub fn to_algebraic_notation(&self) -> String {
        let file = self.file();
        let rank = self.rank();
        let file_char = ('a' as u8 + file as u8) as char;
        let rank_char = ('1' as u8 + rank as u8) as char;
        format!("{}{}", file_char, rank_char)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::assert_matches::assert_matches;

    #[test]
    fn from_algebraic_notation() {
        assert_matches!(Square::from_algebraic_notation("a1").unwrap().index, 56);
        assert_matches!(Square::from_algebraic_notation("b1").unwrap().index, 57);
        assert_matches!(Square::from_algebraic_notation("h8").unwrap().index, 7);

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

    #[test]
    fn rank_and_file() {
        let a1 = Square::from_algebraic_notation("a1").unwrap();
        let h8 = Square::from_algebraic_notation("h8").unwrap();
        let f3 = Square::from_algebraic_notation("f3").unwrap();

        assert_eq!((a1.file(), a1.rank()), (0, 0));
        assert_eq!((h8.file(), h8.rank()), (7, 7));
        assert_eq!((f3.file(), f3.rank()), (5, 2));
    }
}
