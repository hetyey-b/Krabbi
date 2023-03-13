use actix_web::{get, App, HttpResponse, HttpServer, Responder};
use dotenv::dotenv;

pub mod game;

#[get("/")]
async fn hello() -> impl Responder {
    HttpResponse::Ok().body("Hello world!\n")
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    println!("Loading .env variables...");
    dotenv().ok(); // This line loads the environment variables

    let web_server_ip: &str = &std::env::var("WEB_SERVER_IP").expect("WEB_SERVER_IP missing from .env");
    let web_server_port: u16 = std::env::var("WEB_SERVER_PORT").expect("WEB_SERVER_PORT missing from .env").parse::<u16>().unwrap();

    println!("Starting web server at http://{}:{}/", web_server_ip, web_server_port);

    HttpServer::new(|| {
        App::new()
            .service(hello)
    })
        .bind((web_server_ip, web_server_port))?
        .run()
        .await
}
