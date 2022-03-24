#![feature(proc_macro_hygiene, decl_macro)]
#[macro_use] extern crate rocket;

use rocket::response::NamedFile;
use std::path::{Path, PathBuf};

use mongodb::{
    bson::doc,
    sync::Client,
};

// Collection Schema
pub struct Game {
    GameDate: i64,
    gameType: String,
    gameNumber: String,
    Player1Name: String,
    Player2Name: String,
    WinnerName: String,
}

// The is the way we route to certain HTML page
#[get("/")]
fn index() -> Option<NamedFile> {
    NamedFile::open(Path::new("target/deploy/index.html")).ok()
}

fn rocket() -> Result<rocket::Rocket, mongodb::error::Error> { 

    // Get a handle to the collection.
    let collection = Client::with_uri_str("mongodb://localhost:27017")?
                        .database("Connect4DB")
                        .collection::<Game>("games");


    Ok(rocket::ignite().manage(collection).mount("/", routes![index]))
}

fn main() {
    // rocket::ignite().mount("/", routes![index]).launch();
    match rocket() {
        Ok(rocket) => {
            let ret = rocket.launch();
        },
        Err(error) => {
            println!("Failed to create server: {}", error);
        }
    }
}