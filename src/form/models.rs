use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct MissionForm {
    pub missions: Vec<Mission>,
}

/// Mission represents a mission for the competition
#[derive(Serialize, Deserialize, Debug)]
pub struct Mission {
    pub title: String,
    pub sections: Vec<Section>,
}

/// Section represents a field type and its value (score-wise)
#[derive(Serialize, Deserialize, Debug)]
pub struct Section {
    pub field_type: FieldType,
    pub value: i32,
}

#[derive(Serialize, Deserialize, Debug)]
pub enum FieldType {
    StringField(String),
    CheckboxField,
    SelectField,
    RadioField,
}
