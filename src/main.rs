#![feature(plugin, decl_macro)]
#![plugin(rocket_codegen)]

extern crate rocket;
extern crate rocket_contrib;
#[macro_use]
extern crate serde_derive;
extern crate serde_json;

use rocket_contrib::Json;

mod events;
use events::{Organization, Repository, User};
use serde_json::Value;

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

// the meat of the script, will parse
// the json payload and change the label
#[post("/", format = "application/json", data = "<body>")]
fn moved(body: Json<Payload>) -> String {
    if body.action == "moved" {
        return String::from("moved!");
    }
    String::from("Not a supported action")
}

#[get("/")]
fn hello() -> String {
    String::from("Hello, World")
}

fn main() {
    rocket::ignite().mount("/", routes![hello, moved]).launch();
}
