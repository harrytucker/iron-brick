#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use]
extern crate rocket;

#[macro_use]
extern crate rocket_include_static_resources;

use rocket_contrib::database;
use rocket_contrib::{databases::diesel, templates::Template};
use rocket_include_static_resources::StaticResponse;

use lib::{models::*, users::get_users};
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

/// The test function is an example of how to get items from the database
#[get("/test")]
fn test(db: DbConn) -> String {
    let results = match get_users(&db) {
        Ok(users) => users,
        Err(err) => panic!(err),
    };

    format!("First user in db: {}", results[0].username)
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
        .mount("/", routes![index, favicon, test])
        .attach(DbConn::fairing())
        .attach(Template::fairing())
        .attach(StaticResponse::fairing(|resources| {
            static_resources_initialize!(resources, "favicon", "static/lego-brick.png");
        }))
        .manage(missions_str)
        .launch();
}
