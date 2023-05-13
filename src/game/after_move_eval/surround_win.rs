use std::collections::VecDeque;

use crate::game::board::{Board, Tile, Color, HasColor};

pub fn surround_win(board: Board) -> bool {
    let get_king_coords = |get_king_board: Board| -> Result<(usize, usize), String> {
        for i in 0..=10 {
            for j in 0..=10 {
                if get_king_board.get_tile(i, j).unwrap() == Tile::King {
                    return Ok((i, j));
                }
            }
        }
        Err("No King on the board".to_string())
    };

    let king_board_result = get_king_coords(board);
    if king_board_result.is_err() {
        return false;
    }
    let (king_x, king_y) = king_board_result.unwrap();

    let mut flood_fill_visited_coords: Vec<(usize, usize)> = Vec::new();
    let mut surrounded_white_piece_coords: Vec<(usize, usize)> = Vec::new();

    let mut flood_fill = |ff_board: &mut Board, ff_x: usize, ff_y: usize| -> bool {
        if ff_x > 10 || ff_y > 10 {
            return false;
        }
    
        let mut queue: VecDeque<(usize, usize)> = VecDeque::new();
        queue.push_back((ff_x, ff_y));
        while !queue.is_empty() {
            let (x,y) = if let Some((x,y)) = queue.pop_front() { (x,y) } else { break; };
            let current_tile = ff_board.get_tile(x,y).unwrap();

            if x == 0 || x == 10
                || y == 0 || y == 10 {
                return false;
            }

            if current_tile == Tile::Black {
                if x > 0 && x < 10 {
                    let above_tile = ff_board.get_tile(x-1,y).unwrap();
                    let below_tile = ff_board.get_tile(x+1,y).unwrap();
                    let above_safe: bool;
                    let below_safe: bool;
                    above_safe = !flood_fill_visited_coords.contains(&(x-1,y))
                                || above_tile.color() == Color::Black;
                    below_safe = !flood_fill_visited_coords.contains(&(x+1,y))
                                || below_tile.color() == Color::Black;

                    if !above_safe && !below_safe {
                        ff_board.set_tile(Tile::Empty, x, y);
                        queue.push_back((x,y));
                    }
                }
                if y > 0 && y < 10 {
                    let right_tile = ff_board.get_tile(x,y+1).unwrap();
                    let left_tile = ff_board.get_tile(x,y-1).unwrap();
                    let right_safe: bool;
                    let left_safe: bool;
                    right_safe = !flood_fill_visited_coords.contains(&(x,y+1))
                                || right_tile.color() == Color::Black;
                    left_safe = !flood_fill_visited_coords.contains(&(x,y-1))
                                || left_tile.color() == Color::Black;

                    if !right_safe && !left_safe {
                        ff_board.set_tile(Tile::Empty, x, y);
                        queue.push_back((x,y));
                    }
                }
            }

            if current_tile.color() == Color::White {
                if !surrounded_white_piece_coords.contains(&(x,y)) {
                    surrounded_white_piece_coords.push((x,y));
                }
            }

            if x > 10 || y > 10 
                || current_tile.color() == Color::Black 
                || flood_fill_visited_coords.contains(&(x,y)) {
                continue;
            } else {
                flood_fill_visited_coords.push((x,y));
                if x > 0 {
                    queue.push_back((x-1, y));
                }
                if y < 10 {
                    queue.push_back((x, y+1));
                }
                if x < 10 {
                    queue.push_back((x+1, y));
                }
                if y > 0 {
                    queue.push_back((x, y-1));
                }
            }
        }
        true
    };

    let mut new_board = board;
    let ff_result = flood_fill(&mut new_board,king_x,king_y);
    if !ff_result {
        return false;
    }

    // if we didn't hit every white piece, return false
    for i in 0..=10 {
        for j in 0..=10 {
            if board.get_tile(i,j).unwrap().color() == Color::White
                && !surrounded_white_piece_coords.contains(&(i,j)) {
                return false;
            }
        }
    }

    true
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_surround_win() {
        let mut board = Board::new();
        for i in 1..=9 {
            board.set_tile(Tile::Black, 1, i);
            board.set_tile(Tile::Black, 9, i);
        }
        for i in 2..=8 {
            board.set_tile(Tile::Black, i, 1);
            board.set_tile(Tile::Black, i, 9);
        }
        board.set_tile(Tile::King, 5, 5);
        board.set_tile(Tile::White, 4, 4);
        board.set_tile(Tile::White, 4, 5);
        board.set_tile(Tile::White, 4, 6);
        board.set_tile(Tile::White, 5, 4);
        board.set_tile(Tile::White, 5, 6);
        board.set_tile(Tile::White, 6, 4);
        board.set_tile(Tile::White, 6, 5);
        board.set_tile(Tile::White, 6, 6);
        assert!(surround_win(board));

        board.set_tile(Tile::White, 0, 1);
        assert!(!surround_win(board));

        board.set_tile(Tile::Empty, 0, 1);
        board.set_tile(Tile::Empty, 1, 3);
        assert!(!surround_win(board));

        board.set_tile(Tile::Black, 2, 3);
        assert!(!surround_win(board));
    }
}
