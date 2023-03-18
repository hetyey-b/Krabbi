use actix_web::{web, get, post, App, HttpResponse, HttpServer, Responder, Result};
use dotenv::dotenv;
use game::{Game, board::{Board, Color, Tile}};
use serde::Deserialize;
use rusqlite::{NO_PARAMS, params, Connection, Result as RusqliteResult};

pub mod game;

fn main() {
    let mut game: Game = Game::new();

    game.print_board();

    println!("--------------------------");

    let move_result = game.make_move(0, 3, 3, 3);

    if move_result.is_err() {
        println!("Illegal move");
    }

    game.print_board();

    println!("");
    println!("{}", game.to_string().len());
}
//
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
