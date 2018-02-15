#![feature(plugin, decl_macro)]
#![plugin(rocket_codegen)]

extern crate rocket;

#[get("/")]
fn hello() -> String {
    String::from("Hello, World!")
}

#[post("/card_moved")]
fn card_moved() -> String {
    String::from("Hello, World!")
}

fn main () {
    rocket::ignite()
        .mount("/", routes![hello, card_moved])
        .launch();
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
