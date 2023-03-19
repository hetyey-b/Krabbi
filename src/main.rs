use actix_web::{web, get, post, App, HttpResponse, HttpServer, Responder, Result};
use dotenv::dotenv;
use game::{Game, board::{Board, Color, Tile}, after_move_eval::after_move_eval};
use serde::Deserialize;
use rusqlite::{NO_PARAMS, params, Connection, Result as RusqliteResult};

pub mod game;

fn main() {
    let mut board = Board::new();
    board.set_tile(Tile::White, 0, 1);
    board.set_tile(Tile::King, 0, 2);
    board.set_tile(Tile::White, 0, 3);
    board.set_tile(Tile::Black, 1, 1);
    board.set_tile(Tile::Black, 1, 2);
    board.set_tile(Tile::Black, 1, 3);
    board.set_tile(Tile::Black, 0, 4);
    let new_board = after_move_eval(board, 1, 3);

    board.print_board();
    println!("---------------------");
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
