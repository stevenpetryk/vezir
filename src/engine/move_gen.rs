use super::{
    data::move_offsets::*, game_move::GameMove, piece::PieceType::*, position::Position,
    square::Square,
};

impl Position {
    pub fn generate_moves(&self) -> Vec<GameMove> {
        let mut all_moves = vec![];

        for (index, occupancy) in self.occupancies.iter().enumerate() {
            let square = Square { index };

            match occupancy {
                Some(piece) => {
                    if piece.player() != self.to_move {
                        continue;
                    };

                    let mut moves = match piece.piece_type() {
                        Pawn => self.generate_pawn_moves(square),
                        Knight => self.generate_knight_moves(square),
                        Bishop => self.generate_bishop_moves(square),
                        Rook => self.generate_rook_moves(square),
                        Queen => self.generate_queen_moves(square),
                        King => self.generate_king_moves(square),
                    };

                    all_moves.append(&mut moves);
                }
                None => continue,
            };
        }

        all_moves
    }

    fn generate_pawn_moves(&self, square: Square) -> Vec<GameMove> {
        vec![]
    }

    fn generate_knight_moves(&self, square: Square) -> Vec<GameMove> {
        KNIGHT_OFFSETS
            .iter()
            .flat_map(|offsets| {
                offsets.iter().map(|offset| {
                    let new_index = (square.index as i8) + offset;

                    match new_index {
                        0..=63 if self.occupancies[new_index as usize].is_none() => {
                            let new_square = Square {
                                index: new_index as usize,
                            };

                            // Ensure the manhattan distance between the squares
                            // is only 3
                            let manhattan_distance =
                                (new_square.rank() as i8 - square.rank() as i8).abs()
                                    + (new_square.file() as i8 - square.file() as i8).abs();

                            if manhattan_distance != 3 {
                                return None;
                            }

                            Some(GameMove {
                                from: Square {
                                    index: square.index,
                                },
                                to: new_square,
                            })
                        }
                        _ => None,
                    }
                })
            })
            .filter_map(|x| x)
            .collect()
    }

    fn generate_bishop_moves(&self, square: Square) -> Vec<GameMove> {
        self.sliding_moves(&square, &BISHOP_OFFSETS)
    }

    fn generate_rook_moves(&self, square: Square) -> Vec<GameMove> {
        self.sliding_moves(&square, &ROOK_OFFSETS)
    }

    fn generate_queen_moves(&self, square: Square) -> Vec<GameMove> {
        self.sliding_moves(&square, &QUEEN_OFFSETS)
    }

    fn generate_king_moves(&self, square: Square) -> Vec<GameMove> {
        self.sliding_moves(&square, &KING_OFFSETS)
    }

    fn sliding_moves(&self, square: &Square, offsets: &OffsetMap) -> Vec<GameMove> {
        offsets
            .iter()
            .flat_map(|direction| {
                direction
                    .iter()
                    .map_while(|offset| match square.offset(offset) {
                        Some(new_square) if self.occupancies[new_square.index].is_none() => {
                            Some(GameMove {
                                from: Square {
                                    index: square.index,
                                },
                                to: new_square,
                            })
                        }
                        _ => None,
                    })
            })
            .collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_generate_moves() {
        let board = Position::from_fen(
            &"rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1".to_string(),
        )
        .unwrap();

        let moves = board.generate_moves();

        // Assert that 1 ply has 20 moves
        assert_eq!(moves.len(), 20);
    }
}
