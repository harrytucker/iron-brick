#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use]
extern crate rocket;

use rocket_contrib::{json::Json, templates::Template};

use lib::models::*;
use quicli::prelude::*;
use rocket::State;

#[get("/example")]
fn mission_example() -> Json<MissionForm> {
    let mission = Mission {
        title: String::from("M01 - Elevated Places"),
        fields: vec![
            Field::CheckboxField(Checkbox {
                text: String::from("The Robot needs to be supported by the bridge:"),
                value: 20,
            }),
            Field::RadioField(Radio {
                text: String::from(
                    "Flag point only available if the bridge portion is successful!",
                ),
                choices: vec![
                    Choice {
                        text: String::from("No flags raised"),
                        value: 0,
                    },
                    Choice {
                        text: String::from("1 flag raised"),
                        value: 15,
                    },
                    Choice {
                        text: String::from("2 flags raised"),
                        value: 30,
                    },
                ],
            }),
            Field::StringField(String::from("It is okay and expected for Robots to collide while trying to earn flag points. When clearly only one robot holds a raised flag, only that Robot scores for that flag."))
        ],
    };

    Json(MissionForm {
        missions: vec![mission],
    })
}

#[get("/")]
fn index(missions: State<MissionForm>) -> Template {
    let context = missions.inner();
    Template::render("index", &context)
}

fn main() {
    let content = read_file("missions.yaml").unwrap();
    let missions_str = lib::yaml_to_missions(&content).unwrap();

    rocket::ignite()
        .mount("/api", routes![mission_example])
        .mount("/", routes![index])
        .attach(Template::fairing())
        .manage(missions_str)
        .launch();
}
