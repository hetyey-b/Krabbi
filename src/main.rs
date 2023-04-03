use actix_web::{web, get, post, App, HttpResponse, HttpServer, Responder, Result};
use dotenv::dotenv;
use game::{Game, board::{Board, Color, Tile}, after_move_eval::after_move_eval};
use serde::{Deserialize, Serialize};
use rusqlite::{params, Connection, Result as RusqliteResult};

use crate::game::after_move_eval::{edge_fort::edge_fort, surround_win::surround_win};

use uuid::Uuid;

pub mod game;

// fn main() {
//     let mut game = Game::new(true, false);
//
//     game.make_move(0,4,3,4);
//     game.print_board();
// }

static DB_NAME: &str = "test.db";

#[derive(Deserialize, Serialize)]
struct NewGameInfo {
    player_name: String,
    bot_black: bool,
}

#[get("/")]
async fn hello() -> impl Responder {
    HttpResponse::Ok().body("Hello world!\n")
}

#[post("/new_game")]
async fn new_game(new_game_info: web::Json<NewGameInfo>) -> Result<String> {
    let conn = Connection::open(DB_NAME).expect(&format!("Failed database connection to {}",DB_NAME).to_owned());
    let new_game: Game = Game::new(!new_game_info.bot_black, new_game_info.bot_black);
    let player_name = &new_game_info.player_name;

    let result = conn.execute(
        "INSERT INTO games (id, game_state, player_name) VALUES (?1, ?2, ?3)",
        [Uuid::new_v4().to_string(), new_game.to_string().unwrap(), player_name.to_string()],
    );
    
    if result.is_ok() {
        Ok(format!("Welcome {}!", new_game_info.player_name))
    } else {
        Err(actix_web::error::ErrorInternalServerError("Could not create new game!"))
    }
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok(); // This line loads the environment variables

    println!("Connecting to database...");
    let conn = Connection::open(DB_NAME).expect(&format!("Failed database connection to {}",DB_NAME).to_owned());
    println!("Connected to {}",DB_NAME);

    println!("Loading .env variables...");

    let web_server_ip: &str = &std::env::var("WEB_SERVER_IP").expect("WEB_SERVER_IP missing from .env");
    let web_server_port: u16 = std::env::var("WEB_SERVER_PORT").expect("WEB_SERVER_PORT missing from .env").parse::<u16>().unwrap();

    println!("Starting web server at http://{}:{}/", web_server_ip, web_server_port);

    conn.execute(
        "CREATE TABLE IF NOT EXISTS games (
            id TEXT PRIMARY KEY UNIQUE,
            game_state TEXT,
            player_name TEXT
        )",
        [],
    ).expect("Failed to create table 'games'");

    HttpServer::new(|| {
        App::new()
            .service(hello)
            .service(new_game)
    })
        .bind((web_server_ip, web_server_port))?
        .run()
        .await
}
