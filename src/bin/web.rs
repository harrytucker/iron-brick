#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use]
extern crate rocket;

use rocket_contrib::{json::Json, templates::Template};

use lib::models::*;
use quicli::prelude::*;
use rocket::State;

#[derive(Serialize)]
struct TemplateContext<'context> {
    parent: &'static str,
    missions: &'context MissionForm,
}

#[get("/")]
fn index(missions: State<MissionForm>) -> Template {
    Template::render(
        "index",
        &TemplateContext {
            parent: "layout",
            missions: &missions,
        },
    )
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
