use game::board::{Tile,Board,Passable};

pub mod game;

fn main() {
    let mut board: Board = Board::new();
    board.set_tile(Tile::White, 4, 5);
    board.print_board();

    println!("");

    if board.get_tile(4, 0).unwrap().passable() {
        println!("passable");
    } else {
        println!("not passable");
    }
}
