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
