#![feature(proc_macro_hygiene, decl_macro)]
#[macro_use] extern crate rocket;

use rocket::response::NamedFile;
use std::path::{Path, PathBuf};

// The is the way we route to certain HTML page
#[get("/")]
fn index() -> Option<NamedFile> {
    NamedFile::open(Path::new("target/deploy/index.html")).ok()
}

fn main() {
    rocket::ignite().mount("/", routes![index]).launch();
}