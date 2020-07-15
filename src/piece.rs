use std::fmt;
use std::ops;

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
pub enum Color {
    White,
    Black,
}

impl ops::Not for Color {
    type Output = Color;

    fn not(self) -> Self::Output {
        match self {
            Color::Black => Color::White,
            Color::White => Color::Black,
        }
    }
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
pub enum MoveType {
    Normal,
    Capture,
}

// enum for _complete_ movement in any direction e.g. the queen has all of these
#[derive(Debug, PartialEq, Clone, Copy)]
pub enum Direction {
    Horizontal,
    Diagonal,
}

#[derive(Debug, PartialEq, Clone, Copy)]
pub enum Movement<'a> {
    Tuples(&'a [(i32, i32)]),
    Directions(&'a [Direction]),
}

#[derive(Debug, PartialEq, Clone, Copy)]
pub struct Piece {
    name: Name,
    color: Color,
    pub has_moved: bool,
}

impl Piece {
    pub const fn new(name: Name, color: Color) -> Piece {
        Piece {
            name,
            color,
            has_moved: false,
        }
    }

    pub fn is_move_valid(&self, source: (i32, i32), destination: (i32, i32)) -> bool {
        self.is_valid(source, destination, MoveType::Normal)
    }

    pub fn is_capture_valid(&self, source: (i32, i32), destination: (i32, i32)) -> bool {
        self.is_valid(source, destination, MoveType::Capture)
    }

    pub fn is_valid(
        &self,
        source: (i32, i32),
        destination: (i32, i32),
        move_type: MoveType,
    ) -> bool {
        let (x, y) = source;
        let (dest_x, dest_y) = destination;
        match if move_type == MoveType::Normal {
            self.movement()
        } else {
            self.capture()
        } {
            Movement::Tuples(ts) => ts.iter().any(|(mut dx, mut dy)| {
                if self.color == Color::White {
                    dx = -dx;
                    dy = -dy;
                }
                x + dx == dest_x && y + dy == dest_y
            }),
            Movement::Directions(dirs) => {
                for dir in dirs {
                    match dir {
                        Direction::Horizontal if x == dest_x || y == dest_y => return true,
                        Direction::Diagonal if (dest_x - x).abs() == (dest_y - y).abs() => {
                            return true
                        }
                        _ => (),
                    };
                }
                false
            }
        }
    }

    pub fn movement(&self) -> Movement {
        match self.name {
            // movement 1 space in any direction
            Name::King => Movement::Tuples(&[
                (0, -1),
                (1, -1),
                (1, 0),
                (1, 1),
                (0, 1),
                (-1, 1),
                (-1, 0),
                (-1, -1),
            ]),
            Name::Queen => Movement::Directions(&[Direction::Horizontal, Direction::Diagonal]),
            Name::Bishop => Movement::Directions(&[Direction::Diagonal]),
            // just google this if you don't know it
            Name::Knight => Movement::Tuples(&[
                (1, -2),
                (2, -1),
                (2, 1),
                (1, 2),
                (-1, 2),
                (-2, 1),
                (-2, -1),
                (-1, -2),
            ]),
            Name::Rook => Movement::Directions(&[Direction::Horizontal]),
            Name::Pawn => Movement::Tuples(if self.has_moved {
                &[(0, 1)]
            } else {
                &[(0, 1), (0, 2)]
            }),
        }
    }

    pub fn capture(&self) -> Movement {
        if self.name == Name::Pawn {
            Movement::Tuples(&[(1, 1), (-1, 1)])
        } else {
            self.movement()
        }
    }

    pub fn name(&self) -> Name {
        self.name
    }

    pub fn color(&self) -> Color {
        self.color
    }
}

impl fmt::Display for Piece {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{} ",
            match self.color {
                Color::White => match self.name {
                    Name::King => '♔',
                    Name::Queen => '♕',
                    Name::Rook => '♖',
                    Name::Bishop => '♗',
                    Name::Knight => '♘',
                    Name::Pawn => '♙',
                },
                Color::Black => match self.name {
                    Name::King => '♚',
                    Name::Queen => '♛',
                    Name::Rook => '♜',
                    Name::Bishop => '♝',
                    Name::Knight => '♞',
                    Name::Pawn => '♟',
                },
            }
        )
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
                name: Name::King,
                color: Color::White,
                has_moved: false,
            }
        );
    }
    #[test]
    fn test_is_move_valid() {
        assert!(Piece::new(Name::Pawn, Color::Black).is_move_valid((0, 0), (0, 1)));
        assert!(Piece::new(Name::Pawn, Color::White).is_move_valid((0, 1), (0, 0)));
        let white_bishop = Piece::new(Name::Bishop, Color::Black);
        assert!(white_bishop.is_move_valid((0, 1), (5, 6)));
        assert!(!white_bishop.is_move_valid((0, 1), (0, 6)));
        let white_rook = Piece::new(Name::Rook, Color::White);
        assert!(white_rook.is_move_valid((0, 0), (10, 0)));
        assert!(!white_rook.is_move_valid((0, 0), (5, 5)));
    }
    #[test]
    fn test_is_capture_valid() {
        assert!(Piece::new(Name::Pawn, Color::White).is_capture_valid((0, 1), (1, 0)));
        assert!(Piece::new(Name::Pawn, Color::Black).is_capture_valid((0, 1), (1, 2)));
    }
}
