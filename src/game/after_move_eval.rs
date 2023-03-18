use super::board::{Board, HasColor, Color, Tile, Captures};

fn captures_in_dir(board: Board, x: usize, y: usize, x_offset: isize, y_offset: isize) -> bool {
    let captured_x = usize::try_from(isize::try_from(x).unwrap() + x_offset).unwrap();
    let captured_y = usize::try_from(isize::try_from(y).unwrap() + y_offset).unwrap();
    let assisting_x = usize::try_from(isize::try_from(x).unwrap() + (x_offset * 2)).unwrap();
    let assisting_y = usize::try_from(isize::try_from(y).unwrap() + (y_offset * 2)).unwrap();

    let capturing_piece_result = board.get_tile(x, y);
    let captured_piece_result = board.get_tile(captured_x, captured_y);
    let assisting_piece_result = board.get_tile(assisting_x, assisting_y);

    if capturing_piece_result.is_err() 
        || captured_piece_result.is_err() 
        || assisting_piece_result.is_err() {
        return false;
    }

    let capturing_piece = capturing_piece_result.unwrap();
    let captured_piece = captured_piece_result.unwrap();
    let assisting_piece = assisting_piece_result.unwrap();

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

    return king_capture_assisting_up == Tile::Black
            && king_capture_assisting_ri == Tile::Black
            && king_capture_assisting_do == Tile::Black
            && king_capture_assisting_le == Tile::Black;
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
    
    // Check for white escape fort
    
    new_board
}
