pub trait ChessFigure {
    pub fn move_figure();
}

enum Player {
    WHITE,
    BLACK,
}

enum Figures {
    PAWN,
    BISHOP,
    KNIGHT,
    ROOK,
    QUEEN,
    KING,
}

impl ChessFigure for PAWN {
    fn move_figure() {
        todo!();
    }
}

impl ChessFigure for BISHOP {
    fn move_figure() {
        todo!();
    }
}

pub struct Board {
    board: Vec<Vec<u8>>,
    player: Player,
}
