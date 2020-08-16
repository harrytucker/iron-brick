#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}

pub mod models;
use models::*;

pub fn json_to_missions(json: &str) -> Result<MissionForm, serde_json::Error> {
    serde_json::from_str(json)
}

pub fn yaml_to_missions(yaml: &str) -> Result<MissionForm, serde_yaml::Error> {
    serde_yaml::from_str(yaml)
}

pub fn missions_to_json(mission_form: &MissionForm) -> Result<String, serde_json::Error> {
    // pretty print since we want this to be human-readable/editable
    serde_json::to_string_pretty(&mission_form)
}

pub fn missions_to_yaml(mission_form: &MissionForm) -> Result<String, serde_yaml::Error> {
    serde_yaml::to_string(&mission_form)
}
