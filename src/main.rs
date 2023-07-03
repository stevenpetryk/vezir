#![feature(inline_const)]
#![feature(assert_matches)]

mod engine;

fn main() {
    let board = engine::position::Position::from_fen(
        &"rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1".to_string(),
    )
    .unwrap();
    println!("{:?}", board);
}
