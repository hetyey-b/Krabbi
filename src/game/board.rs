struct BoardState {
    board: [[u8; 11]; 11],
}

impl BoardState {
    fn new() -> BoardState {
        let mut new_board = [[0;11]; 11];

        new_board[0][0] = 1;
        new_board[0][11] = 1;
        new_board[11][0] = 1;
        new_board[11][11] = 1;
        new_board[5][5] = 2;

        BoardState { 
            board: new_board,
        }
    }

    fn print_board(&self) {
        for i in 0..self.board.len() {
            for j in 0..self.board[i].len() {
                match self.board[i][j] {
                    0 => print!("0"),
                    1 | 2 => print!("X"),
                    _ => print!(" "),
                }
            }
            println!("");
        }
    }
}
