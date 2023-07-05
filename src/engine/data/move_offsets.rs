use lazy_static::lazy_static;

pub type OffsetMap = Vec<Vec<i8>>;

lazy_static! {
    pub static ref ROOK_OFFSETS: OffsetMap = {
        vec![
            vec![8, 16, 24, 32, 40, 48, 56],
            vec![1, 2, 3, 4, 5, 6, 7],
            vec![-8, -16, -24, -32, -40, -48, -56],
            vec![-1, -2, -3, -4, -5, -6, -7],
        ]
    };
    pub static ref BISHOP_OFFSETS: OffsetMap = vec![
        vec![9, 18, 27, 36, 45, 54, 63],
        vec![7, 14, 21, 28, 35, 42, 49],
        vec![-9, -18, -27, -36, -45, -54, -63],
        vec![-7, -14, -21, -28, -35, -42, -49],
    ];
    pub static ref QUEEN_OFFSETS: OffsetMap = vec![
        vec![8, 16, 24, 32, 40, 48, 56],
        vec![1, 2, 3, 4, 5, 6, 7],
        vec![-8, -16, -24, -32, -40, -48, -56],
        vec![-1, -2, -3, -4, -5, -6, -7],
        vec![9, 18, 27, 36, 45, 54, 63],
        vec![7, 14, 21, 28, 35, 42, 49],
        vec![-9, -18, -27, -36, -45, -54, -63],
        vec![-7, -14, -21, -28, -35, -42, -49],
    ];
    pub static ref KNIGHT_OFFSETS: OffsetMap = vec![
        vec![17],
        vec![15],
        vec![10],
        vec![6],
        vec![-17],
        vec![-15],
        vec![-10],
        vec![-6],
    ];
    pub static ref KING_OFFSETS: OffsetMap = vec![
        vec![8],
        vec![1],
        vec![-8],
        vec![-1],
        vec![9],
        vec![7],
        vec![-9],
        vec![-7],
    ];
}
