/**
 * MongoDB Tutorial
 * https://www.mongodb.com/developer/languages/rust/rust-mongodb-crud-tutorial/
 */

// #[macro_use] extern crate juniper;

// use juniper::{FieldResult};

// #[derive(GraphQLObject)]
// #[graphql(description="A managing account")]
// struct Account {
//     name: String
// }

use mongodb::{Client, options::{ClientOptions}};
use std::error::Error;
use std::env;
use tokio;


#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    // Load mongoDB connection string
    let client_uri = env::var("MONGODB_URI").expect("You must set the MONGODB_URI environment variable!");

    // Parse a connection string into an options struct
    let client_options = ClientOptions::parse(client_uri).await?;

    // Get a handle to the deployment
    let client = Client::with_options(client_options)?;

    // Print the databases in the Cluster
    println!("Databases:");
    for name in client.list_database_names(None, None).await? {
        println!("- {}", name);
    }

    Ok(())
}
