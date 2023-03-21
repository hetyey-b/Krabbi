use std::collections::VecDeque;

use crate::game::{board::{Board, Tile, Color, HasColor}, legal_moves::get_legal_moves};

pub fn surround_win(board::Board) -> bool {
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

    let mut flood_fill = |ff_board: &Board, ff_x: usize, ff_y: usize| {
        if ff_x > 10 || ff_y > 10 {
            return;
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
                if x > &0 && x < &10 {
                    let above_tile = board.get_tile(x-1,*y).unwrap();
                    let below_tile = board.get_tile(x+1,*y).unwrap();
                    let above_safe: bool;
                    let below_safe: bool;
                    above_safe = !flood_fill_visited_coords.contains(&(x-1,*y))
                                || above_tile.color() == Color::Black;
                    below_safe = !flood_fill_visited_coords.contains(&(x+1,*y))
                                || below_tile.color() == Color::Black;

                    if !above_safe && !below_safe {
                        ff_board.set_tile(Tile::Empty, x, y);
                    }
                }
                if y > &0 && y < &10 {
                    let right_tile = board.get_tile(*x,y+1).unwrap();
                    let left_tile = board.get_tile(*x,y-1).unwrap();
                    let right_safe: bool;
                    let left_safe: bool;
                    right_safe = !flood_fill_visited_coords.contains(&(*x,y+1))
                                || right_tile.color() == Color::Black;
                    left_safe = !flood_fill_visited_coords.contains(&(*x,y-1))
                                || left_tile.color() == Color::Black;

                    if !right_safe && !left_safe {
                        ff_board.set_tile(Tile::Empty, x, y);
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
    };

    flood_fill(board,king_x,king_y);

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