use crate::game::board::{Board, Color, HasColor, Tile, Captures, CapturesKing};


pub fn captures_in_dir(board: Board, x: usize, y: usize, x_offset: isize, y_offset: isize) -> bool {
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

    if captured_piece != Tile::King {
        return capturing_piece.captures(captured_piece.color())
                && assisting_piece.captures(captured_piece.color());
    }

    if captured_x <= 0 || captured_x >= 10 ||
        captured_y <= 0 || captured_x >= 10 {
        // if the king is on the edge, it cannot be captured 
        // (by traditional capture)
        return false;
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
