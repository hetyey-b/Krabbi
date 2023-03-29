use super::{board::{Board, Color, HasColor, Tile}, legal_moves::{get_legal_moves, get_all_legal_moves}};
use rand::seq::SliceRandom; 
use std::cmp;

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

fn minimax_alpha_beta(state: &Board, depth: i32, mut alpha: f32, mut beta: f32, max_player: bool) -> f32 {
    if depth == 0 {
        return evaluate(state);
    }

    if max_player {
        let mut max_val = std::f32::MIN;

        // TODO: sub-state generation
        for child_state in get_all_legal_moves(&state).expect("Get all legal moves error") {
            let child_val = minimax_alpha_beta(&child_state, depth-1, alpha, beta, false);

            max_val = f32::max(max_val,child_val);
            alpha = f32::max(alpha, max_val);

            if beta <= alpha {
                break;
            }
        }

        return max_val;
    } else {
        let mut min_val = std::f32::MAX;

        for 
        todo!()
    }
}

fn evaluate(state: &Board) -> f32 {
    let mut result: f32 = 0f32;

    let mut piece_difference: f32 = 0f32;
    let piece_difference_weight = 1f32;
    for i in 0..=10 {
        for j in 0..=10 {
            match state.get_tile(i,j).unwrap() {
                Tile::White => {piece_difference += 2f32},
                Tile::Black => {piece_difference -= 1f32},
                _ => {},
            };
        }
    }
    result += piece_difference * piece_difference_weight;


    result
}
