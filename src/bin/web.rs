#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use]
extern crate rocket;

#[macro_use]
extern crate rocket_include_static_resources;

use rocket_contrib::database;
use rocket_contrib::{databases::diesel, templates::Template};
use rocket_include_static_resources::StaticResponse;

use lib::models::*;
use quicli::prelude::*;
use rocket::State;

/// To configure the database, edit the `global.databases` item in Rocket.toml.
/// You can find the Rocket documentation for this here: https://rocket.rs/v0.4/guide/state/#databases
#[database("sqlite")]
struct DbConn(diesel::SqliteConnection);

#[derive(Serialize)]
struct TemplateContext<'context> {
    parent: &'static str,
    missions: &'context MissionForm,
}

#[get("/favicon.ico")]
fn favicon() -> StaticResponse {
    static_response!("favicon")
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
        .mount("/", routes![index, favicon])
        .attach(DbConn::fairing())
        .attach(Template::fairing())
        .attach(StaticResponse::fairing(|resources| {
            static_resources_initialize!(resources, "favicon", "static/lego-brick.png");
        }))
        .manage(missions_str)
        .launch();
}
