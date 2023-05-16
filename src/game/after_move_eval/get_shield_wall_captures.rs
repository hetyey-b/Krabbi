use crate::game::board::{Board, HasColor, Color, Tile};
use std::collections::VecDeque;

/*
    Given board and coordinates of the last move (x,y), return a vector
    including a tuple of (usize,usize), that indicates the coordinates of every
    enemy piece that has to be removed due to a shieldwall capture
*/
pub fn get_shield_wall_captures(board: Board, x: usize, y: usize) -> Vec<(usize,usize)> {
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
            let (x,y) = if let Some((x,y)) = queue.pop_front() { (x,y) } else { break; };
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
