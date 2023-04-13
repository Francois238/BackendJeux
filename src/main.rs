use actix_cors::Cors;
use dotenvy::dotenv;
use actix_web::{ App, HttpServer, http};

mod api_error;
mod db;
mod schema;
mod game;

#[actix_web::main]
async fn main() -> std::io::Result<()> {

    dotenv().ok();
    env_logger::init();


    HttpServer::new(|| {

        let cors = Cors::default()
        .allowed_origin("http://localhost:4200")
        .allowed_origin("http://129.151.247.65:4200")
        .allowed_methods(vec!["GET", "POST", "PATCH", "DELETE"])
        .allowed_headers(vec![http::header::AUTHORIZATION, http::header::ACCEPT])
        .allowed_header(http::header::CONTENT_TYPE)
        .max_age(3600);

    
        App::new()
        .wrap(cors)
            .configure(game::routes_user)
    })
    .bind(("0.0.0.0", 8080))?
    .run()
    .await
}