use actix_web::{web, get, post, App, HttpResponse, HttpServer, Responder, Result};
use dotenv::dotenv;
use game::{Game, board::{Board, Color, Tile}, after_move_eval::after_move_eval};
use serde::Deserialize;
use rusqlite::{params, Connection, Result as RusqliteResult};

use crate::game::after_move_eval::{edge_fort::edge_fort, surround_win::surround_win};

pub mod game;

fn main() {
    let mut board = Board::new();
    for i in 1..=9 {
        board.set_tile(Tile::Black, 1, i);
        board.set_tile(Tile::Black, 9, i);
    }
    for i in 2..=8 {
        board.set_tile(Tile::Black, i, 1);
        board.set_tile(Tile::Black, i, 9);
    }
    board.set_tile(Tile::King, 5, 5);
    board.set_tile(Tile::White, 4, 4);
    board.set_tile(Tile::White, 4, 5);
    board.set_tile(Tile::White, 4, 6);
    board.set_tile(Tile::White, 5, 4);
    board.set_tile(Tile::White, 5, 6);
    board.set_tile(Tile::White, 6, 4);
    board.set_tile(Tile::White, 6, 5);
    board.set_tile(Tile::White, 6, 6);

    board.set_tile(Tile::White, 0, 1);

    board.set_tile(Tile::Empty, 0, 1);
    board.set_tile(Tile::Empty, 1, 3);

    board.set_tile(Tile::Black, 2, 3);

    board.print_board();
    println!("");

    let cht_fen = board.to_string().unwrap();
    println!("CHT-FEN: {}",cht_fen);

    let new_board = Board::from_string(cht_fen).unwrap();

    println!("");
    println!("--------------");
    new_board.print_board();
}

// static DB_NAME: &str = "test.db";
//
// #[derive(Deserialize)]
// struct NewGameInfo {
//     player_color: String,
//     player_name: String,
// }
//
// #[get("/")]
// async fn hello() -> impl Responder {
//     HttpResponse::Ok().body("Hello world!\n")
// }
//
// #[post("/new_game")]
// async fn new_game(new_game_info: web::Json<NewGameInfo>) -> Result<String> {
//     Ok(format!("Welcome {}!", new_game_info.player_name))
// }
//
// #[actix_web::main]
// async fn main() -> std::io::Result<()> {
//
//     println!("Connecting to database...");
//     let conn = Connection::open(DB_NAME).expect(&format!("Failed database connection to {}",DB_NAME).to_owned());
//     println!("Connected to {}",DB_NAME);
//
//     println!("Loading .env variables...");
//     dotenv().ok(); // This line loads the environment variables
//
//     let web_server_ip: &str = &std::env::var("WEB_SERVER_IP").expect("WEB_SERVER_IP missing from .env");
//     let web_server_port: u16 = std::env::var("WEB_SERVER_PORT").expect("WEB_SERVER_PORT missing from .env").parse::<u16>().unwrap();
//
//     println!("Starting web server at http://{}:{}/", web_server_ip, web_server_port);
//
//     HttpServer::new(|| {
//         App::new()
//             .service(hello)
//             .service(new_game)
//     })
//         .bind((web_server_ip, web_server_port))?
//         .run()
//         .await
// }
