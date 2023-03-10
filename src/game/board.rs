#[derive(Clone, Copy)]
pub enum Tile {
    Empty,
    Corner,
    Black,
    White,
    King,
    ThroneWithKing,
    ThroneEmpty,
}

#[derive(PartialEq)]
pub enum Color {
    Black,
    White,
}

pub trait Captures {
    fn captures(&self, color: Color) -> bool;
}

impl Captures for Tile {
    fn captures(&self, color:Color) -> bool {
        match &self {
            Tile::Empty => false,
            Tile::Corner => true,
            Tile::Black => color == Color::White,
            Tile::White => color == Color::Black,
            Tile::King => color == Color::Black,
            Tile::ThroneWithKing => color == Color::Black,
            Tile::ThroneEmpty => true,
        }
    }
}

pub trait Passable {
    fn passable(&self) -> bool;
}

impl Passable for Tile {
    fn passable(&self) -> bool {
        match &self {
            Tile::Empty | Tile::ThroneEmpty => true,
            Tile::Black | Tile::White | Tile::King | Tile::ThroneWithKing | Tile::Corner => false,
        }
    }
}

pub trait CanStandOn {
    fn can_stand_on(&self) -> bool;
}

impl CanStandOn for Tile {
    fn can_stand_on(&self) -> bool {
        match &self {
            Tile::Empty => true,
            _ => false,
        }
    }
}

struct BoardState {
    board: [[Tile; 11]; 11],
}

impl BoardState {
    fn new() -> BoardState {
        let mut new_board = [[Tile::Empty;11]; 11];

        // set the corner points
        new_board[0][0] = Tile::Corner;
        new_board[0][11] = Tile::Corner;
        new_board[11][0] = Tile::Corner;
        new_board[11][11] = Tile::Corner;
        // set the throne
        new_board[5][5] = Tile::ThroneEmpty;

        BoardState { 
            board: new_board,
        }
    }

    fn get_tile(&self, x:usize, y:usize) -> Tile {
        self.board[x][y] 
    }

    fn set_tile(&mut self, new_tile:Tile, x:usize, y:usize) {
        self.board[x][y] = new_tile;
    }

    fn print_board(&self) {
        for i in 0..self.board.len() {
            for j in 0..self.board[i].len() {
                match self.board[i][j] {
                    Tile::Empty => print!("0"),
                    Tile::Corner | Tile::ThroneEmpty => print!("X"),
                    Tile::Black => print!("B"),
                    Tile::White => print!("W"),
                    Tile::King | Tile::ThroneWithKing => print!("K"),
                }
            }
            println!("");
        }
    }
}
