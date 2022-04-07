#![feature(proc_macro_hygiene, decl_macro)]
#[macro_use] extern crate rocket;

use rocket::response::NamedFile;
use rocket::State;
use std::path::{Path, PathBuf};
use bson::{bson, Bson};
use rocket_contrib::json::Json;
use serde::{Serialize, Deserialize};

use std::io::{self, Read};

use rocket::Data;
use rocket::response::Debug;

use mongodb::{
    bson::doc,
    sync::Client,
    sync::Collection,
};

// Collection Schema
#[derive(Serialize, Deserialize, Debug)]
#[allow(non_snake_case)]
pub struct Game {
    pub GameDate: i64,
    pub gameType: String,
    pub gameNumber: String,
    pub Player1Name: String,
    pub Player2Name: String,
    pub WinnerName: String,
}

//--------------------------------------------------------------------------------

// The is the way we route to certain HTML page
#[get("/")]
fn index() -> Option<NamedFile> {
    NamedFile::open(Path::new("target/deploy/index.html")).ok()
}

// routing for static files
#[get("/<file..>")]
fn files(file: PathBuf) -> Option<NamedFile> {
    NamedFile::open(Path::new("target/deploy/").join(file)).ok()
}

#[get("/games/game_data")]
fn get(collection: State<Collection<Game>>) -> Json<Vec<Game>> {
    let mut games = Vec::new();
    
    //get all the documents in the collection
    if let Ok(mut cursor) = collection.find(doc!{}, None) {

        while let Some(game) = cursor.next() {
            // gloo::console::log!("666666666666676");
            if let Ok(game_doc) = game {
                // gloo::console::log!(&format!("{}", game_doc.Player1Name));
                games.push(game_doc);         //TODO: Need to check whether the game_doc here is parsed as Game correctly or is stll bson style
            }
        }
    }

    Json(games)
}

#[post("/games", format="json", data="<game_json>")]
fn post(game_json: Json<Game>, collection: State<Collection<Game>>) {

    if let game = game_json.into_inner() {
        collection.insert_one(game, None).unwrap();
    }
    
}

fn rocket() -> Result<rocket::Rocket, mongodb::error::Error> { 

    // Get a handle to the collection.
    let collection = Client::with_uri_str("mongodb://localhost:27017")?
                        .database("Connect4DB")
                        .collection::<Game>("games");


    Ok(rocket::ignite().manage(collection).mount("/", routes![index, files, get, post]))
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