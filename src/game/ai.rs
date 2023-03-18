use super::{board::{Board, Color, HasColor}, legal_moves::get_legal_moves};
use rand::seq::SliceRandom; 

pub fn get_random_move(board: Board, color: Color) -> Result<[(usize,usize);2], String> {
    let mut all_legal_moves: Vec<[(usize,usize);2]> = Vec::new();

    for x in 0..11 {
        for y in 0..11 {
            let current_tile_color = &board.get_tile(x,y).unwrap().color();

            if current_tile_color == &color {
                let current_piece_legal_moves_result = get_legal_moves(&board, x, y);

                if current_piece_legal_moves_result.is_err() {
                    continue;
                }

                let current_piece_legal_moves = current_piece_legal_moves_result.unwrap();

                for to in current_piece_legal_moves.iter()  {
                    all_legal_moves.push([(x,y), *to]);
                }
            }
        }
    }

    if all_legal_moves.len() == 0 {
        return Err("No legal moves".to_string());
    }

    Ok(*all_legal_moves.choose(&mut rand::thread_rng()).unwrap())
}
