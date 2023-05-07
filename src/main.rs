use actix_web::{web, get, post, App, HttpResponse, HttpServer, Responder, Result};
use dotenv::dotenv;
use game::{Game, board::{Board, Color, Tile}, after_move_eval::after_move_eval};
use serde::{Deserialize, Serialize};
use rusqlite::{params, Connection, Result as RusqliteResult, Statement};
use crate::game::legal_moves::get_legal_moves;
use actix_web::error::ErrorInternalServerError;
use actix_cors::Cors;

use crate::game::after_move_eval::{edge_fort::edge_fort, surround_win::surround_win};

use uuid::Uuid;

pub mod game;

// fn main() {
//     let mut board: Board = Board::new();
//     board.set_tile(Tile::King, 0, 5);
//     board.print_board();
//     let legal_moves = get_legal_moves(&board, 0, 5).unwrap();
//     println!("legal moves: {:?}", legal_moves);
// }

static DB_NAME: &str = "test.db";

#[derive(Deserialize, Serialize)]
struct NewGameInfo {
    player_name: String,
    bot_white: bool,
    bot_black: bool,
    bot_difficulty: u8,
}

#[derive(Deserialize, Serialize)]
struct MakeMoveInfo {
    player_name: String,
    game_id: String,
    x_from: usize,
    y_from: usize,
    x_to: usize,
    y_to: usize,
}

#[derive(Deserialize, Serialize)]
struct GetLegalMovesInfo {
    player_name: String,
    game_id: String,
    x: usize,
    y: usize,
}

#[derive(Deserialize, Serialize)]
struct GetGamesInfo {
    player_name: String,
}

#[derive(Deserialize, Serialize)]
struct GetBoardInfo {
    game_id: String,
}

#[derive(Deserialize, Serialize)]
struct BoardInfoResponse {
    fen: String,
    winner: String,
}

#[get("/api")]
async fn hello() -> impl Responder {
    HttpResponse::Ok().body("Hello world!\n")
}

#[post("/api/new_game")]
async fn new_game(new_game_info: web::Json<NewGameInfo>) -> Result<String> {
    let conn = Connection::open(DB_NAME).expect(&format!("Failed database connection to {}",DB_NAME).to_owned());

    if new_game_info.bot_white && new_game_info.bot_black {
        return Err(actix_web::error::ErrorInternalServerError("Cannot create a game without a human player!"));
    }

    let bot_difficulty = &new_game_info.bot_difficulty;
    let new_game: Game = Game::new(
                        new_game_info.bot_white, 
                        new_game_info.bot_black, 
                        new_game_info.bot_difficulty
                    );
    let player_name = &new_game_info.player_name;
    let new_game_id = Uuid::new_v4().to_string(); 
    let winner: String = match new_game.get_winner() {
        Color::White => "w".to_string(),
        Color::Black => "b".to_string(),
        Color::None => "x".to_string(),
    };
    if bot_difficulty > &3 || bot_difficulty< &1 {
        return Err(actix_web::error::ErrorInternalServerError("Bot difficulty must be 1, 2 or 3!"));
    }

    let result = conn.execute(
        "INSERT INTO games (id, game_state, player_name, winner, bot_difficulty) 
            VALUES (?1, ?2, ?3, ?4, ?5)",
        [
            &new_game_id, 
            &new_game.to_string().unwrap(), 
            &player_name.to_string(), 
            &winner, 
            &bot_difficulty.to_string()
        ],
    );
   
    if result.is_ok() {
        Ok(new_game_id)
    } else {
        let err = result.unwrap_err();
        println!("{:?}", err);
        Err(actix_web::error::ErrorInternalServerError("Could not create new game!"))
    }
}

#[post("/api/make_move")]
async fn make_move(make_move_info: web::Json<MakeMoveInfo>) -> Result<HttpResponse, actix_web::error::Error> {
    let conn = Connection::open(DB_NAME).expect(&format!("Failed database connection to {}",DB_NAME).to_owned());

    if make_move_info.x_from > 10 
        || make_move_info.y_from > 10
        || make_move_info.x_to > 10
        || make_move_info.y_to > 10 {
        return Err(actix_web::error::ErrorInternalServerError("Incorrect index!"));
    }

    let statement_result = conn.prepare("SELECT * FROM games WHERE id=?1 AND player_name=?2");

    if statement_result.is_err() {
        return Err(actix_web::error::ErrorInternalServerError("SQL error"));
    }

    let mut statement = statement_result.unwrap();

    let rows_result = statement.query(rusqlite::params![make_move_info.game_id, make_move_info.player_name]);

    if rows_result.is_err() {
        return Err(actix_web::error::ErrorInternalServerError("Database query error"));
    }

    let mut rows = rows_result.unwrap();

    let chfen: String;
    let difficulty: u8;

    if let Some(row) = rows.next().transpose() {
        if row.is_err() {
            return Err(actix_web::error::ErrorInternalServerError("No game found"));
        }
        let row_data = row.unwrap();
        chfen = row_data.get("game_state").unwrap();
        difficulty = row_data.get("bot_difficulty").expect("No bot difficulty in db row");
    } else {
        return Err(actix_web::error::ErrorInternalServerError("No game found"));
    }

    let game_result = Game::from_string(chfen, difficulty);

    if game_result.is_err() {
        return Err(actix_web::error::ErrorInternalServerError("Error parsing FEN"));
    }

    let mut game = game_result.unwrap();

    match game.make_move(make_move_info.x_from, make_move_info.y_from, make_move_info.x_to, make_move_info.y_to) {
        Ok(_) => {
            let new_fen = game.to_string().unwrap();

            let winner_char = match game.get_winner() {
                Color::White => 'w'.to_string(),
                Color::Black => 'b'.to_string(),
                Color::None => 'x'.to_string(),
            };
            let update_result = conn.prepare("UPDATE games SET game_state=?1, winner=?2 WHERE id=?3 AND player_name=?4");

            if update_result.is_err() {
                return Err(actix_web::error::ErrorInternalServerError("SQL error"));
            }

            let mut update = update_result.unwrap();

            let update_query_result = update.execute(rusqlite::params![new_fen, winner_char, make_move_info.game_id, make_move_info.player_name]);

            if update_query_result.is_err() {
                return Err(actix_web::error::ErrorInternalServerError("Unable to update database!".to_string()));
            }

            Ok(HttpResponse::Ok().json(BoardInfoResponse {
                fen: new_fen,
                winner: winner_char,
            }))
        },
        Err(err) => {
            // if err == "Game is over!" {
            //     return Ok(HttpResponse::Ok().json(BoardInfoResponse {
            //         fen: new_fen,
            //         winner: winner_char,
            //     }));
            // }

            Err(actix_web::error::ErrorInternalServerError(format!("Invalid move: {}", err)))
        },
    }
}

#[post("/api/legal_moves")]
async fn legal_moves(legal_moves_info: web::Json<GetLegalMovesInfo>) -> Result<String> {
    let conn = Connection::open(DB_NAME).expect(&format!("Failed database connection to {}",DB_NAME).to_owned());

    if legal_moves_info.x > 10 
        || legal_moves_info.y > 10 {
        return Err(actix_web::error::ErrorInternalServerError("Incorrect index!"));
    }

    let statement_result = conn.prepare("SELECT * FROM games WHERE id=?1 AND player_name=?2");

    if statement_result.is_err() {
        return Err(actix_web::error::ErrorInternalServerError("SQL error"));
    }

    let mut statement = statement_result.unwrap();

    let rows_result = statement.query(rusqlite::params![legal_moves_info.game_id, legal_moves_info.player_name]);

    if rows_result.is_err() {
        return Err(actix_web::error::ErrorInternalServerError("Database query error"));
    }

    let mut rows = rows_result.unwrap();

    let chfen: String;
    let difficulty: u8;

    if let Some(row) = rows.next().transpose() {
        if row.is_err() {
            return Err(actix_web::error::ErrorInternalServerError("No game found"));
        }
        let row_data = row.unwrap();
        chfen = row_data.get("game_state").unwrap();
        difficulty = row_data.get("bot_difficulty").expect("No bot difficulty in db row");
    } else {
        return Err(actix_web::error::ErrorInternalServerError("No game found"));
    }

    let game_result = Game::from_string(chfen, difficulty);

    if game_result.is_err() {
        return Err(actix_web::error::ErrorInternalServerError("Error parsing FEN"));
    }

    let game = game_result.unwrap();
    let legal_moves_result = get_legal_moves(&game.board, legal_moves_info.x, legal_moves_info.y);

    if legal_moves_result.is_err() {
        return Err(actix_web::error::ErrorInternalServerError("Error getting legal moves"));
    }

    return Ok(format!("{:?}",legal_moves_result.unwrap()))
}

#[post("/api/get_games")]
async fn get_games(legal_moves_info: web::Json<GetGamesInfo>) -> Result<HttpResponse, actix_web::error::Error> {
    let conn = Connection::open(DB_NAME).expect(&format!("Failed database connection to {}",DB_NAME).to_owned());

    let statement_result = conn.prepare("SELECT id FROM games WHERE player_name=?1");

    if statement_result.is_err() {
        return Err(actix_web::error::ErrorInternalServerError("SQL error"));
    }

    let mut statement = statement_result.unwrap();

    let rows_result = statement.query_map(rusqlite::params![legal_moves_info.player_name], |row| {
        row.get::<usize, String>(0)
    });

    if rows_result.is_err() {
        return Err(actix_web::error::ErrorInternalServerError("Database query error"));
    }

    let rows: Vec<String> = rows_result.unwrap().map(|r| r.unwrap()).collect();

    Ok(HttpResponse::Ok().json(rows))
}

#[post("/api/get_board")]
async fn get_board(get_board_info: web::Json<GetBoardInfo>) -> Result<HttpResponse, actix_web::error::Error> {
    let conn = Connection::open(DB_NAME).expect(&format!("Failed database connection to {}",DB_NAME).to_owned());

    let statement_result = conn.prepare("SELECT * FROM games WHERE id=?1");

    if statement_result.is_err() {
        return Err(actix_web::error::ErrorInternalServerError("SQL error"));
    }

    let mut statement = statement_result.unwrap();

    let rows_result = statement.query(rusqlite::params![get_board_info.game_id]);

    if rows_result.is_err() {
        return Err(actix_web::error::ErrorInternalServerError("Database query error"));
    }

    let mut rows = rows_result.unwrap();

    let chfen: String;
    let winner: String;

    if let Some(row) = rows.next().transpose() {
        if row.is_err() {
            return Err(actix_web::error::ErrorInternalServerError("No game found"));
        }
        let row_data = row.unwrap();
        chfen = row_data.get("game_state").unwrap();
        winner = row_data.get("winner").unwrap();
    } else {
        return Err(actix_web::error::ErrorInternalServerError("No game found"));
    }

    Ok(HttpResponse::Ok().json(BoardInfoResponse {
        fen: chfen,
        winner: winner,
    }))
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
            player_name TEXT,
            winner TEXT,
            bot_difficulty INTEGER
        )",
        [],
    ).expect("Failed to create table 'games'");

    HttpServer::new(|| {
        App::new()
            .wrap(Cors::permissive())
            .service(hello)
            .service(new_game)
            .service(make_move)
            .service(legal_moves)
            .service(get_games)
            .service(get_board)
    })
        .bind((web_server_ip, web_server_port))?
        .run()
        .await
}
