use super::{board::{Board, Color, HasColor, Tile}, legal_moves::{get_legal_moves}, after_move_eval::after_move_eval};
use rand::seq::SliceRandom; 

pub fn get_random_move(board: Board, color: Color) -> Result<[(usize,usize);2], String> {
    let mut all_legal_moves: Vec<[(usize,usize);2]> = Vec::new();

    for x in 0..11 {
        for y in 0..11 {
            let current_tile = &board.get_tile(x,y).expect("Error getting tile");
            let current_tile_color = &current_tile.color();

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

    let mut max_move: [(usize,usize);2] = [(0,0),(0,0)];
    let mut max_value: f32 = f32::MIN;
    let mut min_move: [(usize,usize);2] = [(0,0),(0,0)];
    let mut min_value: f32 = f32::MAX;
    for piece in piece_coords.iter() {
        let current_tile = state.get_tile(piece.0,piece.1)
            .expect("Error in getting tile");
        let current_legal_moves = get_legal_moves(state, piece.0, piece.1)
            .expect("Error in getting legal moves");

        if current_tile == Tile::King && current_player == Color::White {
            if current_legal_moves.contains(&(0,0)) {
                max_move = [piece.clone(), (0,0)];
                break;
            }
            if current_legal_moves.contains(&(10,0)) {
                max_move = [piece.clone(), (10,0)];
                break;
            }
            if current_legal_moves.contains(&(0,10)) {
                max_move = [piece.clone(), (0,10)];
                break;
            }
            if current_legal_moves.contains(&(10,10)) {
                max_move = [piece.clone(), (10,10)];
                break;
            }
        }

        for current_move in current_legal_moves.iter() {
            let mut new_board = state.clone();
            new_board.set_tile(Tile::Empty, piece.0, piece.1);
            new_board.set_tile(current_tile, current_move.0, current_move.1);
            new_board = after_move_eval(new_board, current_move.0, current_move.1);
            let value = minimax_alpha_beta(&new_board, depth, f32::MIN, f32::MAX, current_player == Color::White);

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
        return max_move;
    } else {
        return min_move;
    }
}

// initially, alpha should be f32::MIN, and beta should be f32::MAX
fn minimax_alpha_beta(state: &Board, depth: i32, mut alpha: f32, mut beta: f32, max_player: bool) -> f32 {
    if depth == 0 {
        if evaluate(state) != 0.0 {
            state.print_board();
            println!("evaluation: {}",evaluate(state));
            println!("///////////////////");
        }
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

const BLACK_WEIGHTS: [[f32; 11]; 11] = [
    [0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0],
    [0.0, 5.0, 5.0, 3.0, 3.0, 3.0, 3.0, 3.0, 5.0, 5.0, 0.0],
    [0.0, 5.0, 4.0, 0.0, 0.0, 0.0, 0.0, 0.0, 4.0, 5.0, 0.0],
    [0.0, 3.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 3.0, 0.0],
    [0.0, 3.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 3.0, 0.0],
    [0.0, 3.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 3.0, 0.0],
    [0.0, 3.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 3.0, 0.0],
    [0.0, 3.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 3.0, 0.0],
    [0.0, 5.0, 4.0, 0.0, 0.0, 0.0, 0.0, 0.0, 4.0, 5.0, 0.0],
    [0.0, 5.0, 5.0, 3.0, 3.0, 3.0, 3.0, 3.0, 5.0, 5.0, 0.0],
    [0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0]
];

const WHITE_WEIGHTS: [[f32; 11]; 11] = [
    [0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0],
    [0.0, 5.0, 5.0, 0.0, 0.0, 0.0, 0.0, 0.0, 5.0, 5.0, 0.0],
    [0.0, 5.0, 4.0, 0.0, 0.0, 0.0, 0.0, 0.0, 4.0, 5.0, 0.0],
    [0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0],
    [0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0],
    [0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0],
    [0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0],
    [0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0],
    [0.0, 5.0, 4.0, 0.0, 0.0, 0.0, 0.0, 0.0, 4.0, 5.0, 0.0],
    [0.0, 5.0, 5.0, 0.0, 0.0, 0.0, 0.0, 0.0, 5.0, 5.0, 0.0],
    [0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0]
];
const KING_WEIGHTS: [[f32; 11]; 11] = [
    [f32::MAX, 4.0, 4.0, 4.0, 4.0, 4.0, 4.0, 4.0, 4.0, 4.0, f32::MAX],
    [4.0, 3.0, 3.0, 3.0, 3.0, 3.0, 3.0, 3.0, 3.0, 3.0, 4.0],
    [4.0, 3.0, 2.0, 2.0, 2.0, 2.0, 2.0, 2.0, 2.0, 3.0, 4.0],
    [4.0, 3.0, 2.0, 1.0, 1.0, 1.0, 1.0, 1.0, 2.0, 3.0, 4.0],
    [4.0, 3.0, 2.0, 1.0, 0.0, 0.0, 0.0, 1.0, 2.0, 3.0, 4.0],
    [4.0, 3.0, 2.0, 1.0, 0.0, 1.0, 0.0, 1.0, 2.0, 3.0, 4.0],
    [4.0, 3.0, 2.0, 1.0, 0.0, 0.0, 0.0, 1.0, 2.0, 3.0, 4.0],
    [4.0, 3.0, 2.0, 1.0, 1.0, 1.0, 1.0, 1.0, 2.0, 3.0, 4.0],
    [4.0, 3.0, 2.0, 2.0, 2.0, 2.0, 2.0, 2.0, 2.0, 3.0, 4.0],
    [4.0, 3.0, 3.0, 3.0, 3.0, 3.0, 3.0, 3.0, 3.0, 3.0, 4.0],
    [f32::MAX, 4.0, 4.0, 4.0, 4.0, 4.0, 4.0, 4.0, 4.0, 4.0, f32::MAX]
];

fn evaluate(state: &Board) -> f32 {
    fn get_king_neighbor_weight(state: &Board, x: usize, y: usize) -> f32 {
        if state.get_tile(x, y).expect("Error getting tile result") != Tile::King {
            return 0f32;
        }

        let mut weight: f32 = 0f32;

        if x > 0 {
            weight += match state.get_tile(x-1, y).expect("Error getting tile result") {
                Tile::White => 1f32,
                Tile::Black => -1f32,
                Tile::Empty => 2f32,
                Tile::Throne => 3f32,
                Tile::Corner => 4f32,
                _ => 0f32,
            }; 
        }

        if x > 10 {
            weight += match state.get_tile(x+1, y).expect("Error getting tile result") {
                Tile::White => 1f32,
                Tile::Black => -1f32,
                Tile::Empty => 2f32,
                Tile::Throne => 3f32,
                Tile::Corner => 4f32,
                _ => 0f32,
            }; 
        }

        if y > 0 {
            weight += match state.get_tile(x, y-1).expect("Error getting tile result") {
                Tile::White => 1f32,
                Tile::Black => -1f32,
                Tile::Empty => 2f32,
                Tile::Throne => 3f32,
                Tile::Corner => 4f32,
                _ => 0f32,
            }; 
        }

        if y > 10 {
            weight += match state.get_tile(x, y+1).expect("Error getting tile result") {
                Tile::White => 1f32,
                Tile::Black => -1f32,
                Tile::Empty => 2f32,
                Tile::Throne => 3f32,
                Tile::Corner => 4f32,
                _ => 0f32,
            }; 
        }

        weight
    }

    let mut result: f32 = 0f32;
    
    if state.winner == Color::White {
        return f32::MAX;
    }
    if state.winner == Color::Black {
        return f32::MIN;
    }

    let mut piece_difference: f32 = 0f32;
    let piece_difference_weight = 1f32;

    let mut piece_position: f32 = 0f32;
    let piece_position_weight = 0.75f32;

    let mut king_neighbors: f32 = 0f32;
    let king_neighbors_weight = 0.8f32;

    let mut king_freedom: f32 = 0f32;
    let king_freedom_weight = 1f32;

    for i in 0..=10 {
        for j in 0..=10 {
            match state.get_tile(i,j).unwrap() {
                Tile::White => {
                    piece_difference += 2f32;
                    piece_position += WHITE_WEIGHTS[i][j];
                },
                Tile::Black => {
                    piece_difference -= 1f32;
                    piece_position -= BLACK_WEIGHTS[i][j];
                },
                Tile::King => {
                    piece_position += KING_WEIGHTS[i][j];
                    king_neighbors += get_king_neighbor_weight(state, i, j);
                    king_freedom += get_legal_moves(state, i, j)
                        .expect("Error getting king legal moves").len() as f32;
                },
                _ => {},
            };
        }
    }
    result += (piece_difference * piece_difference_weight)
            + (piece_position * piece_position_weight)
            + (king_neighbors * king_neighbors_weight)
            + (king_freedom * king_freedom_weight);

    result
}
