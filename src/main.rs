#![feature(inline_const)]
#![feature(assert_matches)]

use crate::engine::{game_move::GameMove, square::Square};

mod engine;

fn main() {
    let board = engine::position::Position::from_fen(
        &"rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1".to_string(),
    )
    .unwrap();

    let board = board.apply_move(GameMove {
        from: Square::from_algebraic_notation("e2").unwrap(),
        to: Square::from_algebraic_notation("e4").unwrap(),
    });

    let board = board.apply_move(GameMove {
        from: Square::from_algebraic_notation("e7").unwrap(),
        to: Square::from_algebraic_notation("e5").unwrap(),
    });

    let board = board.apply_move(GameMove {
        from: Square::from_algebraic_notation("g1").unwrap(),
        to: Square::from_algebraic_notation("f3").unwrap(),
    });

    println!("{}", board);
}
