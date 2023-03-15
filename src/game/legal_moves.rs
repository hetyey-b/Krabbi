use std::cmp::{min,max};
use super::board::{Board, Tile, CanStandOn, Passable};

pub fn is_legal_move(board: &Board, x_from: usize, y_from: usize, x_to:usize, y_to:usize) -> bool {
    if x_from > 10 || y_from > 10 || x_to > 10 || y_to > 10 {
        return false;
    }

    let from_result = board.get_tile(x_from, y_from);
    let to_result = board.get_tile(x_to, y_to);

    if from_result.is_err() || to_result.is_err() {
        return false;
    }

    let from = from_result.unwrap();
    let to = to_result.unwrap();

    if from == Tile::Empty || from == Tile::ThroneEmpty || from == Tile::Corner {
        return false;
    }

    if !to.can_stand_on() && from != Tile::King && from != Tile::ThroneWithKing {
        return false;
    }

    if (x_from != x_to) && (y_from != y_to) {
        return false;
    }

    for x in min(x_to,x_from)..=max(x_to,x_from) {
        for y in min(y_to,y_from)..=max(y_to,y_from) {
            if x == x_from && y == y_from {
                // we don't want to check the tile that the piece being moved is on
                continue;
            }

            let current_result = board.get_tile(x, y);

            if current_result.is_err() {
                return false;
            }

            let current = current_result.unwrap();

            if !current.passable() {
                return false;
            }
        }
    }

    return true;
}

pub fn get_legal_moves(board: Board, x: usize, y: usize) -> Result<Vec<(usize,usize)>, String> {
    /*
        1) check if there is a unit on the board at the coordinates
        2) go in each for direction
        3) check if it's passable
            -> if not, break
        4) check if it can be stood on 
            -> if yes, add to the list
    */
    match board.get_tile(x,y) {
        Ok(piece_to_move) => {
            match piece_to_move {
                Tile::Black | Tile::White | Tile::King | Tile::ThroneWithKing => {
                    let mut valid_moves: Vec<(usize,usize)> = Vec::new();
                    
                    // Up 
                    for i in y+1..11 {
                        match board.get_tile(x, i) {
                            Ok(tile) => {
                                if !tile.passable() {
                                    break;
                                }

                                if  tile.can_stand_on() 
                                    || piece_to_move == Tile::King 
                                    || piece_to_move == Tile::ThroneWithKing 
                                {
                                    valid_moves.push((x,i));
                                }
                            },
                            Err(e) => {
                                return Err(e);
                            }
                        }
                    }
                    // Right
                    for i in x+1..11 {
                        match board.get_tile(i, y) {
                            Ok(tile) => {
                                if !tile.passable() {
                                    break;
                                }

                                if  tile.can_stand_on() 
                                    || piece_to_move == Tile::King 
                                    || piece_to_move == Tile::ThroneWithKing 
                                {
                                    valid_moves.push((i,y));
                                }
                            },
                            Err(e) => {
                                return Err(e);
                            }
                        }
                    }
                    // Down
                    for i in (0..y).rev() {
                        match board.get_tile(x, i) {
                            Ok(tile) => {
                                if !tile.passable() {
                                    break;
                                }

                                if  tile.can_stand_on() 
                                    || piece_to_move == Tile::King 
                                    || piece_to_move == Tile::ThroneWithKing 
                                {
                                    valid_moves.push((x,i));
                                }
                            },
                            Err(e) => {
                                return Err(e);
                            }
                        }
                    }
                    // Left 
                    for i in (0..x).rev() {
                        match board.get_tile(i, y) {
                            Ok(tile) => {
                                if !tile.passable() {
                                    break;
                                }

                                if  tile.can_stand_on() 
                                    || piece_to_move == Tile::King 
                                    || piece_to_move == Tile::ThroneWithKing 
                                {
                                    valid_moves.push((i,y));
                                }
                            },
                            Err(e) => {
                                return Err(e);
                            }
                        }
                    }

                    return Ok(valid_moves);
                },
                _ => {
                    return Err("No piece at coordinates".to_string());
                }
            }
        },
        Err(e) => {
            return Err(e);
        },
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_indexing_err() {
       assert!(get_legal_moves(Board::new(), 11, 4).is_err());
       assert!(get_legal_moves(Board::new(), 4, 11).is_err());
       // Without piece
       assert!(get_legal_moves(Board::new(), 4, 4).is_err());
    }

    #[test]
    fn test_legal_moves_empty_board() {
        let mut board: Board = Board::new();
        board.set_tile(Tile::King, 4, 4);
        
        let legal_moves = get_legal_moves(board, 4, 4).unwrap();
        
        assert_eq!(legal_moves.len(), 20);

        assert!(legal_moves.contains(&(0,4)));
        assert!(legal_moves.contains(&(1,4)));
        assert!(legal_moves.contains(&(2,4)));
        assert!(legal_moves.contains(&(3,4)));

        assert!(legal_moves.contains(&(5,4)));
        assert!(legal_moves.contains(&(6,4)));
        assert!(legal_moves.contains(&(7,4)));
        assert!(legal_moves.contains(&(8,4)));
        assert!(legal_moves.contains(&(9,4)));
        assert!(legal_moves.contains(&(10,4)));
        
        assert!(legal_moves.contains(&(4,0)));
        assert!(legal_moves.contains(&(4,1)));
        assert!(legal_moves.contains(&(4,2)));
        assert!(legal_moves.contains(&(4,3)));

        assert!(legal_moves.contains(&(4,5)));
        assert!(legal_moves.contains(&(4,6)));
        assert!(legal_moves.contains(&(4,7)));
        assert!(legal_moves.contains(&(4,8)));
        assert!(legal_moves.contains(&(4,9)));
        assert!(legal_moves.contains(&(4,10)));

        assert_eq!(legal_moves.contains(&(4,4)), false);
        assert_eq!(legal_moves.contains(&(5,5)), false);
        assert_eq!(legal_moves.contains(&(3,3)), false);
        assert_eq!(legal_moves.contains(&(2,2)), false);
    }

    #[test]
    fn test_legal_moves_for_piece() {
        let mut board: Board = Board::new();
        board.set_tile(Tile::White, 4, 5);
        board.set_tile(Tile::Black, 3, 5);
        board.set_tile(Tile::Black, 4, 7);
        board.set_tile(Tile::Black, 7, 5);
        
        let legal_moves = get_legal_moves(board, 4, 5).unwrap();
        
        assert_eq!(legal_moves.len(), 7);

        assert!(legal_moves.contains(&(4,0)));
        assert!(legal_moves.contains(&(4,1)));
        assert!(legal_moves.contains(&(4,2)));
        assert!(legal_moves.contains(&(4,3)));
        assert!(legal_moves.contains(&(4,4)));

        assert!(legal_moves.contains(&(4,6)));

        assert!(legal_moves.contains(&(6,5)));

        assert_eq!(legal_moves.contains(&(5,5)), false);
    }

    #[test]
    fn test_legal_moves_for_king() {
        let mut board: Board = Board::new();
        board.set_tile(Tile::King, 4, 5);
        board.set_tile(Tile::Black, 3, 5);
        board.set_tile(Tile::Black, 4, 7);
        board.set_tile(Tile::Black, 7, 5);
        
        let legal_moves = get_legal_moves(board, 4, 5).unwrap();
        
        assert_eq!(legal_moves.len(), 8);

        assert!(legal_moves.contains(&(4,0)));
        assert!(legal_moves.contains(&(4,1)));
        assert!(legal_moves.contains(&(4,2)));
        assert!(legal_moves.contains(&(4,3)));
        assert!(legal_moves.contains(&(4,4)));

        assert!(legal_moves.contains(&(4,6)));

        assert!(legal_moves.contains(&(6,5)));

        assert!(legal_moves.contains(&(5,5)));
    }

    #[test]
    fn test_legal_move_wrong_index() {
        let mut board: Board = Board::new();
        board.set_tile(Tile::White, 4, 4);

        assert_eq!(is_legal_move(&board, 4, 4, 11, 12), false);
        assert_eq!(is_legal_move(&board, 12, 11, 4, 4), false);
        assert_eq!(is_legal_move(&board, 5, 5, 6, 6), false);
    }

    #[test]
    fn test_legal_move_valid_index() {
        let mut board: Board = Board::new();
        board.set_tile(Tile::White, 4, 5);
        board.set_tile(Tile::Black, 7, 5);

        // TODO
        assert!(is_legal_move(&board, 4, 5, 6, 5));
        assert!(is_legal_move(&board, 4, 5, 0, 5));
        assert!(is_legal_move(&board, 4, 5, 4, 0));
        assert!(is_legal_move(&board, 4, 5, 4, 10));
        // END OF TODO

        assert_eq!(is_legal_move(&board, 4, 5, 5, 5), false);
        assert_eq!(is_legal_move(&board, 4, 5, 7, 5), false);
        assert_eq!(is_legal_move(&board, 4, 5, 8, 5), false);
    }
}
