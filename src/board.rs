use super::piece::{Color, Piece, Name};

const BOARD_SIZE: usize = 8;
type BoardType = [[Option<Piece>; BOARD_SIZE]; BOARD_SIZE];

const DEFAULT_BOARD: BoardType = [
    [
        Some(Piece::new(Name::Rook, Color::Black)),
        Some(Piece::new(Name::Knight, Color::Black)),
        Some(Piece::new(Name::Bishop, Color::Black)),
        Some(Piece::new(Name::Queen, Color::Black)),
        Some(Piece::new(Name::King, Color::Black)),
        Some(Piece::new(Name::Bishop, Color::Black)),
        Some(Piece::new(Name::Knight, Color::Black)),
        Some(Piece::new(Name::Rook, Color::Black)),
    ],
    [Some(Piece::new(Name::Pawn, Color::Black)); BOARD_SIZE],
    [None; BOARD_SIZE],
    [None; BOARD_SIZE],
    [None; BOARD_SIZE],
    [None; BOARD_SIZE],
    [Some(Piece::new(Name::Pawn, Color::White)); BOARD_SIZE],
    [
        Some(Piece::new(Name::Rook, Color::White)),
        Some(Piece::new(Name::Knight, Color::White)),
        Some(Piece::new(Name::Bishop, Color::White)),
        Some(Piece::new(Name::Queen, Color::White)),
        Some(Piece::new(Name::King, Color::White)),
        Some(Piece::new(Name::Bishop, Color::White)),
        Some(Piece::new(Name::Knight, Color::White)),
        Some(Piece::new(Name::Rook, Color::White)),
    ],
];

#[derive(Debug, PartialEq)]
pub struct Board {
    inner: BoardType,
}

impl Board {
    pub const fn new() -> Board {
        Board {
            inner: DEFAULT_BOARD,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new() {
        assert_eq!(Board::new(), Board { inner: DEFAULT_BOARD });
    }
}
