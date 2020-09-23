#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use]
extern crate rocket;

#[macro_use]
extern crate rocket_include_static_resources;

use rocket_contrib::templates::Template;
use rocket_include_static_resources::StaticResponse;

use lib::models::*;
use quicli::prelude::*;
use rocket::State;

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
        .attach(Template::fairing())
        .attach(StaticResponse::fairing(|resources| {
            static_resources_initialize!(resources, "favicon", "static/lego-brick.png");
        }))
        .manage(missions_str)
        .launch();
}
