#![feature(plugin, decl_macro)]
#![plugin(rocket_codegen)]

extern crate futures;
extern crate hyper;
#[macro_use]
extern crate log;
extern crate rocket;
extern crate rocket_contrib;
#[macro_use]
extern crate serde_derive;
extern crate serde_json;
extern crate tokio_core;

use rocket_contrib::Json;

mod events;
use events::{Organization, Repository, User};
use serde_json::Value;
use futures::{Future, Stream};
use std::io::{self, Write};
use hyper::Client;
use tokio_core::reactor::Core;

#[derive(Deserialize)]
#[allow(dead_code)]
struct ProjectCard {
    url: String,
    column_url: String,
    column_id: i32,
    id: i32,
    note: Value,
    creator: User,
    content_url: String,
    created_at: String,
    updated_at: String,
}

#[derive(Deserialize)]
#[allow(dead_code)]
struct Payload {
    action: String,
    project_card: ProjectCard,
    organization: Organization,
    repository: Repository,
    sender: User,
}


enum States {
    TO_DO,
    IN_PROGRESS,
    QA,
    Demo,
    Done
}

// the meat of the script, will parse
// the json payload and change the label
#[post("/", format = "application/json", data = "<body>")]
fn moved(body: Json<Payload>) -> String {
    const GITHUB_BASE_URL : str = "https://api.github.com";

    trace!("action: {}", body.action);
    if body.project_card.note == Value::Null {
        trace!("Not an issue");
        return String::from("Not an issue");
    }
    let _card_created = body.action == "created";
    let mut core = Core::new()?;
    let client = Client::new(&core.handle());
    let uri = GITHUB_BASE_URL & 

    String::from("Not a supported action")
}

#[get("/")]
fn hello() -> String {
    String::from("Hello, World")
}

fn main() {
    rocket::ignite().mount("/", routes![hello, moved]).launch();
}
