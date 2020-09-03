#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use]
extern crate rocket;

use rocket_contrib::{json::Json, templates::Template};

use lib::models::*;
use quicli::prelude::*;
use rocket::State;

#[get("/")]
fn index(missions: State<MissionForm>) -> Template {
    let context = missions.inner();
    Template::render("index", &context)
}

fn main() {
    let content = read_file("missions.yaml").unwrap();
    let missions_str = lib::yaml_to_missions(&content).unwrap();

    rocket::ignite()
        .mount("/", routes![index])
        .attach(Template::fairing())
        .manage(missions_str)
        .launch();
}
