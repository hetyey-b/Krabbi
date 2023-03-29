use std::thread::current;

use self::{legal_moves::is_legal_move, board::Board, board::Color, board::Tile, after_move_eval::after_move_eval};
use crate::game::board::HasColor;

pub mod legal_moves;
pub mod board;
pub mod ai;
pub mod after_move_eval;

const BLACK_COORDS: [(usize, usize); 24] = [
    (0,3),
    (0,4),
    (0,5),
    (0,6),
    (0,7),
    (1,5),
    (3,0),
    (3,10),
    (4,0),
    (4,10),
    (5,0),
    (5,1),
    (5,10),
    (5,9),
    (6,0),
    (6,10),
    (7,0),
    (7,10),
    (9,5),
    (10,3),
    (10,4),
    (10,5),
    (10,6),
    (10,7),
];
const WHITE_COORDS: [(usize,usize);12] = [
    (3,5),
    (4,4),
    (4,5),
    (4,6),
    (5,3),
    (5,4),
    (5,6),
    (5,7),
    (6,4),
    (6,5),
    (6,6),
    (7,5),
];
const KING_COORD: (usize,usize) = (5,5);
/*
    1) create a game with one of the players being AI

    2) when given a move to play, play it if it's legal 
        -> evaluate captures
        -> evaluate win conditions
    3) call the AI for the response move
    4) return the new board state, with the AI move done

    2-4 is one process
    AI model has to come from outside as a parameter
        -> that way we can swap it easily for harder/easier AI
        -> also allows for better testing, since AI can be mocked more easily
*/


pub struct Game {
    pub board: Board,
    pub current_player: Color,
    pub bot_white: bool,
    pub bot_black: bool,
}

impl Game {
    pub fn new(bot_player_white: bool, bot_player_black: bool) -> Game {
        let mut new_board = Board::new();

        for coord in BLACK_COORDS.iter() {
            new_board.set_tile(Tile::Black, coord.0, coord.1);
        }
        for coord in WHITE_COORDS.iter() {
            new_board.set_tile(Tile::White, coord.0, coord.1);
        }
        new_board.set_tile(Tile::King, KING_COORD.0, KING_COORD.1);
        
        Game {
            board: new_board,
            current_player: Color::Black,
            bot_white: bot_player_white,
            bot_black: bot_player_black,
        }
    }

    pub fn print_board(&self) {
        self.board.print_board();
        println!("");
        if self.current_player == Color::Black {
            println!("Next move: Black");
        } else {
            println!("Next move: White");
        }
    }

    pub fn from_string(mut str: String, bot_player_white: bool, bot_player_black: bool) -> Result<Game, String> {
        let new_current_player: Color;
        match str.pop() {
            Some('w') => new_current_player = Color::White,
            Some('b') => new_current_player = Color::Black,
            _ => return Err("Wrong format: unknown player marker".to_string()),
        };

        match str.pop() {
            Some('/') => {},
            _ => return Err("Wrong format".to_string()),
        };

        let new_board_result = Board::from_string(str);

        if new_board_result.is_err() {
            return Err("Wrong format: Error when loading board".to_string());
        }

        Ok(Game {
            board: new_board_result.unwrap(),
            current_player: new_current_player,
            bot_white: bot_player_white,
            bot_black: bot_player_black,
        })
    }

    pub fn to_string(&self) -> Result<String,String> {
        let str_result = self.board.to_string();

        if str_result.is_err() {
            return Err("Couldn't format board".to_string());
        }

        let mut str = str_result.unwrap();
        str.push('/');

        if self.current_player == Color::Black {
            str.push('b');
        } else {
            str.push('w');
        }

        return Ok(str);
    }

    pub fn get_winner(&self) -> Color {
        self.board.winner 
    }

    pub fn make_move(&mut self, x_from: usize, y_from: usize, x_to: usize, y_to: usize) -> Result<&Board, &str> {
        if self.board.winner != Color::None {
            return Err("Game is over!");
        }

        if !is_legal_move(&self.board, x_from, y_from, x_to, y_to) {
            return Err("Illegal move!");
        }

        let from = &self.board.get_tile(x_from, y_from).unwrap();
        // let to = &self.board.get_tile(x_to, y_to).unwrap();

        if &from.color() != &self.current_player {
            return Err("Not the current player!");
        }

        self.board.set_tile(*from, x_to, y_to);

        if x_from == 5 && y_from == 5 {
            self.board.set_tile(Tile::Throne, x_from, y_from);
        } else {
            self.board.set_tile(Tile::Empty, x_from, y_from);
        }

        self.board = after_move_eval(self.board, x_to, y_to);

        if self.current_player == Color::White {
            self.current_player = Color::Black;
        } else {
            self.current_player = Color::White;
        }

        return Ok(&self.board);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_game_string_conversion() {
        let game = Game::new(false, false);

        let string_conversion = game.to_string().unwrap();

        let new_game = Game::from_string(string_conversion, false, false).unwrap();

        assert_eq!(game.get_winner(), new_game.get_winner());
        assert_eq!(game.current_player, new_game.current_player);

        for i in 0..=10 {
            for j in 0..=10 {
                assert!(game.board.get_tile(i,j).is_ok());
                assert!(new_game.board.get_tile(i,j).is_ok());
                assert_eq!(game.board.get_tile(i,j).unwrap(), new_game.board.get_tile(i,j).unwrap());
            }
        }
    }
}
