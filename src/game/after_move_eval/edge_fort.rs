use std::collections::VecDeque;

use crate::game::{board::{Board, Tile, Color, HasColor}, legal_moves::get_legal_moves};

/*
    1) run flood fill on the king
    2) if it hits a black piece, return false
    3) if it never hits an edge, return false
    4) note all White tiles it hit 
    5) check the White tiles:
        -> if 2 opposing sides are:
            - Empty or Black
            - not checked out in search
                => not valid edge fort
*/
pub fn edge_fort(board: Board) -> bool {
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

    if king_x != 0 && king_x != 10
        && king_y != 0 && king_y != 10 {
        return false;
    }

    if get_legal_moves(&board, king_x, king_y).unwrap().len() == 0 {
        return false;
    }

    let mut flood_fill_visited_coords: Vec<(usize, usize)> = Vec::new();
    let mut edge_fort_pieces_coords: Vec<(usize, usize)> = Vec::new();

    let mut flood_fill = |ff_board: Board, ff_x: usize, ff_y: usize| {
        if ff_x > 10 || ff_y > 10 {
            return;
        }
        
        let mut queue: VecDeque<(usize, usize)> = VecDeque::new();
        queue.push_back((ff_x, ff_y));
        while !queue.is_empty() {
            let (x,y) = if let Some((x,y)) = queue.pop_front() { (x,y) } else { break; };
            let current_tile = ff_board.get_tile(x,y).unwrap();

            if current_tile == Tile::Black {
                flood_fill_visited_coords = Vec::new();
                return;
            }

            if current_tile == Tile::White {
                if !edge_fort_pieces_coords.contains(&(x,y)) {
                    edge_fort_pieces_coords.push((x,y));
                }
                continue;
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

    if flood_fill_visited_coords.len() == 0 {
        return false;
    }

    for coords in edge_fort_pieces_coords.iter() {
        let (x,y) = coords;
        if x > &0 && x < &10 {
            let above_tile = board.get_tile(x-1,*y).unwrap();
            let below_tile = board.get_tile(x+1,*y).unwrap();
            let above_safe: bool;
            let below_safe: bool;
            above_safe = flood_fill_visited_coords.contains(&(x-1,*y))
                        || above_tile.color() == Color::White;
            below_safe = flood_fill_visited_coords.contains(&(x+1,*y))
                        || below_tile.color() == Color::White;

            if !above_safe && !below_safe {
                return false;
            }
        }
        if y > &0 && y < &10 {
            let right_tile = board.get_tile(*x,y+1).unwrap();
            let left_tile = board.get_tile(*x,y-1).unwrap();
            let right_safe: bool;
            let left_safe: bool;
            right_safe = flood_fill_visited_coords.contains(&(*x,y+1))
                        || right_tile.color() == Color::White;
            left_safe = flood_fill_visited_coords.contains(&(*x,y-1))
                        || left_tile.color() == Color::White;

            if !right_safe && !left_safe {
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
    fn test_legal_edge_fort() {
        let mut board = Board::new();
        board.set_tile(Tile::White,0,1);
        board.set_tile(Tile::White,0,4);
        board.set_tile(Tile::White,1,2);
        board.set_tile(Tile::White,1,3);
        board.set_tile(Tile::King,0,3);
        assert!(edge_fort(board));

        board.set_tile(Tile::White,1,1);
        board.set_tile(Tile::White,1,4);
        board.set_tile(Tile::White,2,2);
        board.set_tile(Tile::White,2,3);
        assert!(edge_fort(board));
    }

    #[test]
    fn test_illegal_edge_fort() {
        let mut board = Board::new();
        board.set_tile(Tile::White,0,1);
        board.set_tile(Tile::White,0,4);
        board.set_tile(Tile::White,1,2);
        board.set_tile(Tile::White,1,4);
        board.set_tile(Tile::White,2,3);
        board.set_tile(Tile::King,0,3);
        assert!(!edge_fort(board));

        board.set_tile(Tile::White,2,4);
        assert!(edge_fort(board));

        board.set_tile(Tile::Empty,0,3);
        board.set_tile(Tile::King,1,3);
        assert!(!edge_fort(board));

        board.set_tile(Tile::King,0,3);
        board.set_tile(Tile::White,0,2);
        board.set_tile(Tile::White,1,3);
        assert!(!edge_fort(board));

        board.set_tile(Tile::Empty,0,2);
        board.set_tile(Tile::Empty,1,3);
        assert!(edge_fort(board));

        board.set_tile(Tile::Black,1,3);
        assert!(!edge_fort(board));
    }
}
