use super::{board::{Board, Color, HasColor, Tile}, legal_moves::{get_legal_moves}, after_move_eval::after_move_eval};
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

fn get_all_child_boards(board: &Board, player: Color) -> Vec<Board> {
    let mut result: Vec<Board> = Vec::new();

    for i in 0..=10 {
        for j in 0..=10 {
            let current_tile = board.get_tile(i,j).unwrap();
            if current_tile.color() != player {
                continue;
            }

            let moves = get_legal_moves(board, i, j).unwrap();

            for current_move in moves.iter() {
                let mut new_board = board.clone();
                new_board.set_tile(Tile::Empty, i, j);
                new_board.set_tile(current_tile, current_move.0, current_move.1);
                new_board = after_move_eval(new_board, current_move.0, current_move.1);
                result.push(new_board);
            }
        }
    }

    result
}

pub fn minimax_best_move(state: &Board, current_player: Color, depth: i32) -> [(usize,usize);2] {
    let mut piece_coords: Vec<(usize,usize)> = Vec::new();

    for i in 0..=10 {
        for j in 0..=10 {
            if state.get_tile(i, j).unwrap().color() == current_player {
                piece_coords.push((i,j));
            }
        }
    }
    println!("piece coords: {:?}", piece_coords);

    let mut max_move: [(usize,usize);2] = [(0,0),(0,0)];
    let mut max_value: f32 = f32::MIN;
    let mut min_move: [(usize,usize);2] = [(0,0),(0,0)];
    let mut min_value: f32 = f32::MAX;
    for piece in piece_coords.iter() {
        let current_tile = state.get_tile(piece.0,piece.1).unwrap();
        let current_legal_moves = get_legal_moves(state, piece.0, piece.1).unwrap();
        println!("piece {:?} legal moves: {}",piece,current_legal_moves.len());
        for current_move in current_legal_moves.iter() {
            let mut new_board = state.clone();
            new_board.set_tile(Tile::Empty, piece.0, piece.1);
            new_board.set_tile(current_tile, current_move.0, current_move.1);
            new_board = after_move_eval(new_board, current_move.0, current_move.1);
            let value = minimax_alpha_beta(&new_board, depth, f32::MIN, f32::MAX, current_player == Color::White);

            println!("value: {} - move: {:?}", value, [piece.clone(),current_move.clone()]);
            if value > max_value {
                max_move = [piece.clone(),current_move.clone()];
                max_value = value;
            }
            if value < min_value {
                min_move = [piece.clone(),current_move.clone()];
                min_value = value;
            }
        }
    }

    if current_player == Color::White {
        return min_move;
    } else {
        return max_move;
    }
}

// initially, alpha should be f32::MIN, and beta should be f32::MAX
fn minimax_alpha_beta(state: &Board, depth: i32, mut alpha: f32, mut beta: f32, max_player: bool) -> f32 {
    if depth == 0 {
        // state.print_board();
        // println!("evaluation: {}",evaluate(state));
        // println!("///////////////////");
        return evaluate(state);
    }

    if max_player {
        let mut max_val = f32::MIN;

        for child_state in get_all_child_boards(&state, Color::White) {
            let child_val = minimax_alpha_beta(&child_state, depth-1, alpha, beta, false);

            max_val = f32::max(max_val,child_val);
            alpha = f32::max(alpha, max_val);

            if beta <= alpha {
                break;
            }
        }
        return max_val;
    } else {
        let mut min_val = f32::MAX;

        for child_state in get_all_child_boards(&state, Color::Black) {
            let child_val = minimax_alpha_beta(&child_state, depth-1, alpha, beta, true);

            min_val = f32::min(min_val, child_val);
            beta = f32::min(beta, min_val);

            if beta <= alpha {
                break;
            }
        }

        return min_val;
    }
}

fn evaluate(state: &Board) -> f32 {
    let mut result: f32 = 0f32;
    
    if state.winner == Color::White {
        return f32::MAX;
    }
    if state.winner == Color::Black {
        return f32::MIN;
    }

    let mut piece_difference: f32 = 0f32;
    let piece_difference_weight = 1f32;

    for i in 0..=10 {
        for j in 0..=10 {
            match state.get_tile(i,j).unwrap() {
                Tile::White => {
                    piece_difference += 2f32;
                },
                Tile::Black => {
                    piece_difference -= 1f32;
                },
                _ => {},
            };
        }
    }
    result += piece_difference * piece_difference_weight;

    result
}
