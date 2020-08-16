use serde::{Deserialize, Serialize};

/// MissionForm serves as a parent struct that wraps everything.
///
/// This is useful on the client side of things...
#[derive(Serialize, Deserialize, Debug)]
pub struct MissionForm {
    pub missions: Vec<Mission>,
}

/// The Mission type and its childen use the serde serialisation
/// items to provide a nice JSON representation of all the missions in
/// a given competition, and to use it in Rust nicely.
#[derive(Serialize, Deserialize, Debug)]
pub struct Mission {
    pub title: String,
    pub fields: Vec<Field>,
}

/// A mission contains a number of fields, which can be one of the
/// following types
///
/// StringField: effectively a form label for extra info
/// CheckboxField: a checkbox and a text description, can give points
/// SelectField: a drop-down with a text description
/// RadioField: radio buttons with a text description
#[derive(Serialize, Deserialize, Debug)]
pub enum Field {
    StringField(String),
    CheckboxField(Checkbox),
    SelectField(Select),
    RadioField(Radio),
}

/// CheckboxField: a checkbox and a text description, can give points
#[derive(Serialize, Deserialize, Debug)]
pub struct Checkbox {
    pub text: String,
    pub value: i32,
}

/// SelectField: a drop-down with a text description
#[derive(Serialize, Deserialize, Debug)]
pub struct Select {
    pub text: String,
    pub choices: Vec<Choice>,
}

/// RadioField: radio buttons with a text description
#[derive(Serialize, Deserialize, Debug)]
pub struct Radio {
    pub text: String,
    pub choices: Vec<Choice>,
}

/// Choice: generic struct for field types that have multiple choices
#[derive(Serialize, Deserialize, Debug)]
pub struct Choice {
    pub text: String,
    pub value: i32,
}
