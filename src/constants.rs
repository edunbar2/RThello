// -----constants----- //
pub const BOARD_SIZE: usize = 8;
pub const STARTING_BLACK: u64 = 0x0000000810000000;
//black and white stored in binary for ease of consideration
pub const STARTING_WHITE: u64 = 0x0000001008000000;
pub const STARTING_PLAYER: Color = Color::Black;
//Black goes first
pub const EDGE_MASK: u64 = 0xff818181818181ff;
pub const FULL: u64 = 0xffffffffffffffff;
//using mod operator to get remainder. This is to get around shifting by negative values with a usize
pub const DIRECTIONS: [usize; 8] = [19, 18, 17, 11, 1, 7, 8, 9];
#[derive(Clone, Copy)]
pub enum Color{
    Black,
    White
}

pub enum Difficulty{
    None,
    Easy,
    Normal,
    Hard
}

// ***** Board Layout ***** \\
/*
a1 a2 a3 a4 a5 a6 a7 a8
b1 b2 b3 b4 b5 b6 b7 b8
c1 c2 c3 c4 c5 c6 c7 c8
d1 d2 d3 d4 d5 d6 d7 d8
e1 e2 e3 e4 e5 e6 e7 e8
f1 f2 f3 f4 f5 f6 f7 f8
g1 g2 g3 g4 g5 g6 g7 g8
h1 h2 h3 h4 h5 h6 h7 h8
*/