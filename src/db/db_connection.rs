use dotenv::dotenv;
use std::env;

use mongodb::{options::ClientOptions, Client, error::Result};

pub async fn get_db_client() -> Result<Client> {
    dotenv().ok(); // Load the .env file
    
    let database_url = env::var("MONGODB_URL").expect("MONGODB_URL must be set in env variable");
    let mut client_options = ClientOptions::parse(&database_url).await?;

    client_options.app_name = Some("glaflem".to_string());
    client_options.max_pool_size = Some(10);
    client_options.max_connecting = Some(10);
    client_options.min_pool_size = Some(1);

    let client = Client::with_options(client_options)?;
    Ok(client)
}