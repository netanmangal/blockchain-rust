use dotenv::dotenv;
use mongodb::{Client, options::ClientOptions};
use std::error::Error;

pub struct DB {
    pub client: Client,
}

impl DB {
    pub async fn connect_db() -> Result<Self, Box<dyn Error>> {
        dotenv().ok();

        let mongodb_uri: String =
            dotenv::var("MONGODB_URI").expect("Expected env variable: MONGODB_URI");
        let mut client_options: ClientOptions = ClientOptions::parse(mongodb_uri).await?;
        client_options.app_name = Some("Student-Teacher".to_string());

        let client: Client = Client::with_options(client_options)?;

        println!("Database connected.");

        Ok(Self { client })
    }
}