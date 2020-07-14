use std::fmt;

#[derive(Debug, PartialEq, Clone, Copy)]
pub enum Color {
    White,
    Black,
}

#[derive(Debug, PartialEq, Clone, Copy)]
pub enum Name {
    King,
    Queen,
    Rook,
    Bishop,
    Knight,
    Pawn,
}

#[derive(Debug, PartialEq, Clone, Copy)]
pub struct Piece {
    piece_type: Name,
    color: Color,
}

impl Piece {
    pub const fn new(piece_type: Name, color: Color) -> Piece {
        Piece { piece_type, color }
    }
}

impl fmt::Display for Piece {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", match self.color {
            Color::White => match self.piece_type {
                Name::King => '♔',
                Name::Queen => '♕',
                Name::Rook => '♖',
                Name::Bishop => '♗',
                Name::Knight => '♘',
                Name::Pawn => '♙',
            },
            Color::Black => match self.piece_type {
                Name::King => '♚',
                Name::Queen => '♛',
                Name::Rook => '♜',
                Name::Bishop => '♝',
                Name::Knight => '♞',
                Name::Pawn => '♟',
            }
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new() {
        assert_eq!(
            Piece::new(Name::King, Color::White),
            Piece {
                piece_type: Name::King,
                color: Color::White
            }
        );
    }
}
