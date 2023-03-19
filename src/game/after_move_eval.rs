use std::collections::VecDeque;

use super::board::{Board, HasColor, Color, Tile, Captures, CapturesKing};

fn captures_in_dir(board: Board, x: usize, y: usize, x_offset: isize, y_offset: isize) -> bool {
    let captured_x: usize;
    let captured_y: usize;
    let assisting_x: usize;
    let assisting_y: usize;
    {
        let captured_x_result = usize::try_from(isize::try_from(x).unwrap() + x_offset);
        let captured_y_result = usize::try_from(isize::try_from(y).unwrap() + y_offset);
        let assisting_x_result = usize::try_from(isize::try_from(x).unwrap() + (x_offset * 2));
        let assisting_y_result = usize::try_from(isize::try_from(y).unwrap() + (y_offset * 2));

        if captured_x_result.is_err()
            || captured_y_result.is_err()
            || assisting_x_result.is_err()
            || assisting_y_result.is_err() {
            return false;
        }

        captured_x = captured_x_result.unwrap();
        captured_y = captured_y_result.unwrap();
        assisting_x = assisting_x_result.unwrap();
        assisting_y = assisting_y_result.unwrap();
    }

    let capturing_piece;
    let captured_piece;
    let assisting_piece;
    {
        let capturing_piece_result = board.get_tile(x, y);
        let captured_piece_result = board.get_tile(captured_x, captured_y);
        let assisting_piece_result = board.get_tile(assisting_x, assisting_y);

        if capturing_piece_result.is_err() 
            || captured_piece_result.is_err() 
            || assisting_piece_result.is_err() {
            return false;
        }

        capturing_piece = capturing_piece_result.unwrap();
        captured_piece = captured_piece_result.unwrap();
        assisting_piece = assisting_piece_result.unwrap();
    }


    if captured_piece.color() == Color::None {
        return false;
    }

    if captured_piece != Tile::King && captured_piece != Tile::ThroneWithKing {
        return capturing_piece.captures(captured_piece.color())
                && assisting_piece.captures(captured_piece.color());
    }

    // captured piece is a King, we need to check the other assisting tiles
    let king_capture_assisting_up_result = board.get_tile(captured_x - 1, captured_y);
    let king_capture_assisting_ri_result = board.get_tile(captured_x , captured_y + 1);
    let king_capture_assisting_do_result = board.get_tile(captured_x  + 1, captured_y);
    let king_capture_assisting_le_result = board.get_tile(captured_x, captured_y - 1);

    if king_capture_assisting_up_result.is_err() 
        || king_capture_assisting_ri_result.is_err() 
        || king_capture_assisting_do_result.is_err() 
        || king_capture_assisting_le_result.is_err()  {
        return false;
    }

    let king_capture_assisting_up = king_capture_assisting_up_result.unwrap();
    let king_capture_assisting_ri = king_capture_assisting_ri_result.unwrap();
    let king_capture_assisting_do = king_capture_assisting_do_result.unwrap();
    let king_capture_assisting_le = king_capture_assisting_le_result.unwrap();

    return king_capture_assisting_up.captures_king()
            && king_capture_assisting_ri.captures_king()
            && king_capture_assisting_do.captures_king()
            && king_capture_assisting_le.captures_king();
}

fn get_shield_wall_captures(board: Board, x: usize, y: usize) -> Vec<(usize,usize)> {
    /*
        1) See if any neighbor is of opposite color
        2) If so, floodfill on it, looking for it's color
        3) If we hit an Empty, return just an empty Vector
        4) When the fill is done, return an array of everything filled except the king
            -> (The king lives through a shieldwall capture)
    */
    let capturing: Tile;
    let up: Tile;
    let right: Tile;
    let down: Tile;
    let left: Tile;
    {
        let up_result = if x <= 0 { Err("index too small".to_string()) } else { board.get_tile(x-1, y) }; 
        let ri_result = if y >= 9 { Err("index too large".to_string()) } else { board.get_tile(x, y+1) };
        let do_result = if x >= 9 { Err("index too large".to_string()) } else { board.get_tile(x+1, y) };
        let le_result = if y <= 0 { Err("index too small".to_string()) } else { board.get_tile(x, y-1) };
        let capturing_result = board.get_tile(x, y);

        if capturing_result.is_err() {
            return Vec::new();
        } 
        capturing = capturing_result.unwrap();

        up = if up_result.is_ok() {
            up_result.unwrap()
        } else {
            Tile::Empty 
        };
        right = if ri_result.is_ok() {
            ri_result.unwrap()
        } else {
            Tile::Empty 
        };
        down = if do_result.is_ok() {
            do_result.unwrap()
        } else {
            Tile::Empty 
        };
        left = if le_result.is_ok() {
            le_result.unwrap()
        } else {
            Tile::Empty 
        };
    }
    let color = capturing.color();

    if color == Color::None {
        return Vec::new();
    }

    let captured_color = if color == Color::White {
        Color::Black
    } else {
        Color::White
    };

    /*
        1) create a function right inside here
        2) instead of coloring, it adds coords to the vector, and checks whether those coords are in there
    */ 

    let mut flood_filled_tile_coords: Vec<(usize, usize)> = Vec::new();

    let mut flood_fill = |ff_board: Board, ff_x: usize, ff_y: usize, ff_color: Color| {
        if ff_x > 10 || ff_y > 10 {
            return;
        }
        let tile_result = ff_board.get_tile(ff_x, ff_y);
        if tile_result.is_err() {
            return;
        }

        let tile = tile_result.unwrap();
        if tile.color() != ff_color {
            return;
        }
        let mut queue: VecDeque<(usize, usize)> = VecDeque::new();

        queue.push_back((ff_x, ff_y));
        while !queue.is_empty() {
            println!("queue: {:?}",queue);
            let (x,y) = if let Some((x,y)) = queue.pop_front() { (x,y) } else { break; };
            println!("(x,y) = ({},{})",x,y);
            let current_tile = ff_board.get_tile(x,y).unwrap();

            if current_tile == Tile::Empty {
                flood_filled_tile_coords = Vec::new();
                return;
            }

            if x > 10 || y > 10 
                || current_tile.color() != ff_color 
                || flood_filled_tile_coords.contains(&(x,y)) {
                continue;
            } else {
                flood_filled_tile_coords.push((x,y));
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

    if up.color() == captured_color {
       flood_fill(board,x-1,y,captured_color);
    }
    if right.color() == captured_color {
       flood_fill(board,x,y+1,captured_color);
    }
    if down.color() == captured_color {
       flood_fill(board,x+1,y,captured_color);
    }
    if left.color() == captured_color {
       flood_fill(board,x,y-1,captured_color);
    }

    
    flood_filled_tile_coords
}

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

    }

    // Check for white escape fort
    if tile_color == Color::White {

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
