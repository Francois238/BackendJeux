use dotenvy::dotenv;
use actix_web::{ App, HttpServer};

mod api_error;
mod db;
mod schema;
mod game;

#[actix_web::main]
async fn main() -> std::io::Result<()> {

    dotenv().ok();
    env_logger::init();


    HttpServer::new(|| {
        App::new()
            .configure(game::routes_user)
    })
    .bind(("0.0.0.0", 8080))?
    .run()
    .await
}