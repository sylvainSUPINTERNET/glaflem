mod db;
use dotenv::dotenv;
use actix_web::{web, App, HttpServer, Responder};
use db::db_connection::get_db_client;

use mongodb::{options::ClientOptions, Client, error::Result};

async fn index() -> impl Responder {
    "Hello world!"
}


#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok(); // Load the .env file


    let db_client = get_db_client().await;
    let client_ref;

    match db_client {
        Ok(client) => {
            client_ref = client.clone();
            client_ref.list_database_names().await.unwrap().iter().for_each(|db_name| {
                println!("{}", db_name);
            });
            
            println!("Connected to database");
        },
        Err(e) => {
            println!("Failed to connect to database: {:?}", e);
        }
    }

    HttpServer::new(|| {
        App::new().service(
            // prefixes all resources and routes attached to it...
            web::scope("/app")
                // ...so this handles requests for `GET /app/index.html`
                .route("/index.html", web::get().to(index)),
        )
    })
    .bind(("127.0.0.1", 9999))?
    .run()
    .await
}