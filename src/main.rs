#![feature(plugin, decl_macro)]
#![plugin(rocket_codegen)]

extern crate log;
extern crate rocket;
extern crate serde_json;

// the meat of the script, will parse
// the json payload and change the label
// #[post("/", format="application/json", data = "<body>")]
// fn moved(body: Json) -> String {
//     let v: Value = serde_json::from_str(body);
//     info!("body: {}", v);
//     if v["action"] == "moved" {
//         return "moved!";
//     }
//     "Not a supported action"
// }

#[get("/")]
fn hello() -> String {
    String::from("Hello, World")
}

fn main() {
    rocket::ignite().mount("/", routes![hello]).launch();
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
