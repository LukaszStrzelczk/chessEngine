use std::{any::Any, cell};

const BOARD_SIZE: usize = 8;

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum Color {
    White,
    Black,
}

#[warn(non_camel_case_types)]
pub trait Chess_piece: Any {
    fn is_valid_move(&self, from: (usize, usize), to: (usize, usize), board: &Board) -> bool;
    fn get_color(&self) -> Color;
    fn as_any(&self) -> &dyn Any;
}

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum ChessPiece {
    Pawn,
    Bishop,
    Knight,
    Rook,
    Queen,
    King,
}

pub struct Piece {
    piece: ChessPiece,
    color: Color,
}

impl Piece {
    pub fn new(piece: ChessPiece, color: Color) -> Self {
        Piece { piece, color }
    }
}

impl Chess_piece for Piece {
    fn get_color(&self) -> Color {
        self.color
    }
    fn is_valid_move(&self, from: (usize, usize), to: (usize, usize), board: &Board) -> bool {
        match self.piece {
            ChessPiece::Pawn => move_pawn(from, to, board),
            ChessPiece::Bishop => move_bishop(from, to, board),
            ChessPiece::Knight => move_knight(from, to, board),
            ChessPiece::Rook => move_rook(from, to, board),
            ChessPiece::Queen => move_queen(from, to, board),
            ChessPiece::King => move_king(from, to, board),
        }
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
}

pub struct Board {
    board: [[Option<Box<dyn Chess_piece>>; BOARD_SIZE]; BOARD_SIZE],
    current_player: Color,
}

impl Board {
    /// initiates a chess board with all pieces
    pub fn init() -> Board {
        let mut board: [[Option<Box<dyn Chess_piece>>; BOARD_SIZE]; BOARD_SIZE] = Default::default();
        for i in 0..BOARD_SIZE {
            board[1][i] = Some(Box::new(Piece::new(ChessPiece::Pawn, Color::White)));
            board[6][i] = Some(Box::new(Piece::new(ChessPiece::Pawn,Color::Black)));
        }
        use ChessPiece::*;
        let pieces = [Rook,Knight,Bishop,King,Queen,Bishop,Knight,Rook];
        for (idx,piece) in  pieces.iter().enumerate(){
            board[0][idx] = Some(Box::new(Piece::new(*piece, Color::White)));
            board[7][idx] = Some(Box::new(Piece::new(*piece, Color::Black)));
        }

        Board { board, current_player: Color::White }
    }
    /// logic  for moving pieces
    /// # Arguments
    /// * `from` - current piece position
    /// * `to` - position we want to move piece to
    /// 
    pub fn move_piece(&self, from: (usize, usize), to: (usize, usize)){todo!()}

    /// prints board to screen
    pub fn show_board(&self){
        for row in self.board.iter(){
            for cell in row.iter(){
               if let Some(piece) = cell{
                   if let Some(concrete_piece) = piece.as_any().downcast_ref::<Piece>() {
                    let symbol = match concrete_piece.piece {
                        ChessPiece::Rook => "r",
                        ChessPiece::Knight => "n",
                        ChessPiece::Bishop => "b",
                        ChessPiece::Queen => "q",
                        ChessPiece::King => "k",
                        ChessPiece::Pawn => "p",
                    };
                    let symbol = if concrete_piece.get_color()==Color::White {
                        symbol.to_uppercase()
                    }else {
                        symbol.to_lowercase()
                    };
                    print!(" {symbol}")
                   }
               }
               else {
                   print!(" _")
               }
            }
            println!()
        }
    }
}

fn move_pawn(from: (usize, usize), to: (usize, usize), board: &Board) -> bool {
    todo!()
}
fn move_knight(from: (usize, usize), to: (usize, usize), board: &Board) -> bool {
    todo!()
}
fn move_bishop(from: (usize, usize), to: (usize, usize), board: &Board) -> bool {
    todo!()
}
fn move_rook(from: (usize, usize), to: (usize, usize), board: &Board) -> bool {
    todo!()
}
fn move_queen(from: (usize, usize), to: (usize, usize), board: &Board) -> bool {
    todo!()
}
fn move_king(from: (usize, usize), to: (usize, usize), board: &Board) -> bool {
    todo!()
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_name() {
       
    }
}