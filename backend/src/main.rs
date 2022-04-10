#![feature(proc_macro_hygiene, decl_macro)]
#[macro_use] extern crate rocket;

use rocket::response::NamedFile;
use rocket::response::Redirect;
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

// Path redirections, used for handling exceptional cases
#[get("/GameHistory")]
fn game_history_redirect() -> Redirect {
    Redirect::to("/")
}

#[get("/HowToConnect4")]
fn how_to_c4_redirect() -> Redirect {
    Redirect::to("/")
}

#[get("/Connect4Computer")]
fn c4_computer_redirect() -> Redirect {
    Redirect::to("/")
}

#[get("/Connect4Human")]
fn c4_human_redirect() -> Redirect {
    Redirect::to("/")
}

#[get("/HowToToot")]
fn how_to_tt_redirect() -> Redirect {
    Redirect::to("/")
}

#[get("/TootOttoComputer")]
fn tt_computer_redirect() -> Redirect {
    Redirect::to("/")
}

#[get("/TootOttoHuman")]
fn tt_human_redirect() -> Redirect {
    Redirect::to("/")
}

#[get("/Scores")]
fn scores_redirect() -> Redirect {
    Redirect::to("/")
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
            if let Ok(game_doc) = game {
                games.push(game_doc);        
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


    Ok(rocket::ignite().manage(collection).mount("/", routes![index, files, get, post, game_history_redirect, how_to_c4_redirect, c4_computer_redirect, c4_human_redirect, how_to_tt_redirect, tt_computer_redirect, tt_human_redirect, scores_redirect]))
}

fn main() {
    match rocket() {
        Ok(rocket) => {
            let ret = rocket.launch();
        },
        Err(error) => {
            println!("Failed to create server: {}", error);
        }
    }
}