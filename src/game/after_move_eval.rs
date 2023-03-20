use self::edge_fort::edge_fort;

use super::board::{Board, HasColor, Color, Tile};

use crate::game::after_move_eval::captures_in_dir::captures_in_dir;
use crate::game::after_move_eval::get_shield_wall_captures::get_shield_wall_captures;

pub mod captures_in_dir;
pub mod get_shield_wall_captures;
pub mod edge_fort;

/*
    Takes in a board, and the coordinates of the last piece that was moved.
    Returns the new state of the board, with captures evaluated
*/
pub fn after_move_eval(board: Board, x: usize, y: usize) -> Board {
    let mut new_board: Board = board;
    let get_tile_result = board.get_tile(x, y);

    if get_tile_result.is_err() {
        return board;
    }

    let tile = get_tile_result.unwrap();
    let tile_color = tile.color();

    if tile_color == Color::None {
        return board;
    }

    // Check for king escaped
    if tile == Tile::King
        && (x == 0 || x == 10)
        && (y == 0 || y == 10) {
        new_board.winner = Color::White;
        return new_board;
    }

    // Check for shieldwall capture
    let shieldwall_captured_coords = get_shield_wall_captures(board, x, y);

    for coords in shieldwall_captured_coords.iter() {
        if board.get_tile(coords.0, coords.1).unwrap() != Tile::King {
            println!("Removing at ({},{})", coords.0, coords.1);
            new_board.set_tile(Tile::Empty, coords.0, coords.1);
        }
    }

    // Check for immediate captures
    //      > king is captured from 4 sides
    //      > empty throne captures the king
    //      > corner doesn't capture the king
    
    // Up
    if captures_in_dir(board, x, y, -1, 0) {
        new_board.set_tile(Tile::Empty, x-1, y);
    }
    // Right
    if captures_in_dir(board, x, y, 0, 1) {
        new_board.set_tile(Tile::Empty, x, y+1);
    }
    // Down
    if captures_in_dir(board, x, y, 1, 0) {
        new_board.set_tile(Tile::Empty, x+1, y);
    }
    // Left
    if captures_in_dir(board, x, y, 0, -1) {
        new_board.set_tile(Tile::Empty, x, y-1);
    }
    
    // Check for black surrounds white
    if tile_color == Color::Black {
        // TODO
    }

    // Check for white escape fort
    if tile_color == Color::White {
        if edge_fort(board) {
            new_board.winner = Color::White;
        }
    }
    
    new_board
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_simple_captures() {
        let mut board = Board::new();
        board.set_tile(Tile::Black, 7, 1);
        board.set_tile(Tile::Black, 5, 3);
        board.set_tile(Tile::White, 7, 2);
        board.set_tile(Tile::White, 6, 3);
        board.set_tile(Tile::White, 7, 4);
        board.set_tile(Tile::Black, 7, 3);
        let new_board = after_move_eval(board, 7, 3);
        assert_eq!(new_board.get_tile(7,2).unwrap(), Tile::Empty);
        assert_eq!(new_board.get_tile(6,3).unwrap(), Tile::Empty);
        assert_eq!(new_board.get_tile(7,4).unwrap(), Tile::White);
    }

    #[test]
    fn test_empty_throne_capture() {
        let mut board = Board::new();
        board.set_tile(Tile::Black, 6, 5);
        board.set_tile(Tile::White, 7, 5);
        let new_board = after_move_eval(board, 7, 5);
        assert_eq!(new_board.get_tile(6,5).unwrap(), Tile::Empty);
    }

    #[test]
    fn test_full_throne_capture() {
        let mut board = Board::new();
        board.set_tile(Tile::ThroneWithKing, 5, 5);
        board.set_tile(Tile::White, 6, 5);
        board.set_tile(Tile::Black, 7, 5);
        let new_board = after_move_eval(board, 6, 5);
        assert_eq!(new_board.get_tile(6,5).unwrap(), Tile::White);
    }

    #[test]
    fn test_corner_capture() {
        let mut board = Board::new();
        board.set_tile(Tile::White, 0, 1);
        board.set_tile(Tile::Black, 0, 2);
        let new_board = after_move_eval(board, 0, 2);
        assert_eq!(new_board.get_tile(0,1).unwrap(), Tile::Empty);
    }

    #[test]
    fn test_move_between() {
        let mut board = Board::new();
        board.set_tile(Tile::White, 0, 5);
        board.set_tile(Tile::Black, 0, 4);
        board.set_tile(Tile::Black, 0, 6);
        let mut new_board = after_move_eval(board, 0, 5);
        assert_eq!(new_board.get_tile(0,5).unwrap(), Tile::White);

        new_board = after_move_eval(board, 0, 4);
        assert_eq!(new_board.get_tile(0,5).unwrap(), Tile::Empty);
    }

    #[test]
    fn test_king_captures() {
        let mut board = Board::new();
        board.set_tile(Tile::King, 6, 5);
        board.set_tile(Tile::Black, 7, 5);
        board.set_tile(Tile::Black, 6, 4);
        board.set_tile(Tile::Black, 6, 6);
        let mut new_board = after_move_eval(board, 6, 6);
        assert_eq!(new_board.get_tile(6,5).unwrap(), Tile::Empty);

        board.set_tile(Tile::King, 6, 5);
        board.set_tile(Tile::Empty, 7, 5);
        new_board = after_move_eval(board, 6, 6);
        assert_eq!(new_board.get_tile(6,5).unwrap(), Tile::King);
    }

    #[test]
    fn test_shield_wall_capture() {
        let mut board = Board::new();
        board.set_tile(Tile::White, 0, 1);
        board.set_tile(Tile::King, 0, 2);
        board.set_tile(Tile::White, 0, 3);
        board.set_tile(Tile::Black, 1, 1);
        board.set_tile(Tile::Black, 1, 2);
        board.set_tile(Tile::Black, 1, 3);
        board.set_tile(Tile::Black, 0, 4);
        let mut new_board = after_move_eval(board, 1, 3);
        assert_eq!(new_board.get_tile(0,1).unwrap(), Tile::Empty);
        assert_eq!(new_board.get_tile(0,2).unwrap(), Tile::King);
        assert_eq!(new_board.get_tile(0,3).unwrap(), Tile::Empty);

        board = Board::new();
        board.set_tile(Tile::White, 0, 1);
        board.set_tile(Tile::White, 0, 3);
        board.set_tile(Tile::Black, 1, 1);
        board.set_tile(Tile::Black, 1, 2);
        board.set_tile(Tile::Black, 1, 3);
        board.set_tile(Tile::Black, 0, 4);
        new_board = after_move_eval(board, 1, 3);
        assert_eq!(new_board.get_tile(0,1).unwrap(), Tile::White);
        assert_eq!(new_board.get_tile(0,2).unwrap(), Tile::Empty);
        assert_eq!(new_board.get_tile(0,3).unwrap(), Tile::White);
    }
}
