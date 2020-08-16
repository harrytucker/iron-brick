#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use]
extern crate rocket;

extern crate serde;
extern crate serde_json;

use std::fs::File;
use std::path::Path;

use rocket_contrib::json::Json;

mod form;
use form::models::*;

#[get("/")]
fn index() -> Json<MissionForm> {
    Json(MissionForm {
        missions: vec![Mission {
            title: "M01 - The Testing Range".to_string(),
            sections: vec![
                Section {
                    field_type: FieldType::RadioField,
                    value: 10,
                },
                Section {
                    field_type: FieldType::StringField("KONO DIO DA".to_string()),
                    value: 0,
                },
            ],
        }],
    })
}

fn main() {
    rocket::ignite().mount("/", routes![index]).launch();
}
