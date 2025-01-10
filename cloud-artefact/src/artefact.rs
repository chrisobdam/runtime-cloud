use core::str;

use crate::wasi::keyvalue::store;
use serde_derive::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
struct Application {
    uuid: String,
    name: String,
    actions: Vec<Action>,
}

trait GetAction {
    fn get_action(&self, uuid: String) -> Option<&Action>;
}

impl GetAction for Application {
    fn get_action(&self, uuid: String) -> Option<&Action> {
        for action in &self.actions {
            if action.uuid == uuid {
                return Some(action);
            }
        }
        None
    }
}

//this does not seem to be compatible with unwrap_or_default
//but needs to stay here for to be used in the action_exists_in_app function
impl Default for Application {
    fn default() -> Self {
        let app = Application {
            uuid: String::from("default-uuid"),
            name: String::from("default-name"),
            actions: vec![],
        };
        app
    }
}

#[derive(Serialize, Deserialize, Debug)]
struct Action {
    uuid: String,
    auth: String,
    scope: String,
    etag: String,
}

#[derive(Serialize, Deserialize, Debug)]
struct Cloud {
    name: String,
    applications: Vec<Application>,
}
trait GetApplication {
    fn get_application(&self, uuid: String) -> Option<&Application>;
}

impl GetApplication for Cloud {
    fn get_application(&self, uuid: String) -> Option<&Application> {
        for app in &self.applications {
            if app.uuid == uuid {
                return Some(app);
            }
        }
        None
    }
}

#[derive(Serialize, Deserialize, Debug)]
struct Root {
    cloud: Cloud,
}

fn action_exists_in_app(cloud: &Cloud, app_uuid: String, action_uid: String) -> bool {
    //FIX: findout how to define a default trait that unwrap_or_default can use
    let _default_app = Application::default();
    match cloud.get_application(app_uuid) {
        Some(app) => app,
        None => &_default_app,
    }
    .get_action(action_uid)
    .is_some()
}

pub fn validate(app_uuid: String, action_uid: String) -> Result<bool, String> {
    let bucket = store::open("default").unwrap();

    match bucket.get("cloud") {
        Ok(str) => {
            let artefact = String::from_utf8(str.unwrap()).expect("Stored value is not valid utf8");
            let Root { cloud } = serde_json::from_str(&artefact).unwrap();
            match action_exists_in_app(&cloud, app_uuid, action_uid) {
                true => Ok(true),
                false => Err("Action not found".to_string()),
            }
        }
        Err(err) => Err(format!("Error: {}", err)),
    }
}

pub fn write_artefact(artefact: &String) -> Result<String, String> {
    // we don't accept any input for now, so we use the json string below
    let json_str = "{
        \"cloud\": {
            \"name\": \"BettyBlocks\",
            \"applications\": [
                {
                    \"uuid\": \"123\",
                    \"name\": \"MyApp\",
                    \"actions\": [
                        {
                            \"uuid\": \"456\",
                            \"auth\": \"None\",
                            \"scope\": \"Public\"
                        }
                    ]
                }
            ]
        }
    }";

    let bucket = store::open("default").unwrap();

    match bucket.set("cloud", json_str.as_bytes()) {
        Ok(_) => match bucket.get("cloud") {
            Ok(str) => Ok(String::from_utf8(str.unwrap()).unwrap()),
            Err(err) => Err(format!("Error: {}", err)),
        },
        Err(err) => Err(format!("Error: {}", err)),
    }
}

mod tests {
    use super::*;

    #[test]
    fn test_action_exists_in_app() {
        let cloud = Cloud {
            name: "BettyBlocks".to_string(),
            applications: vec![Application {
                uuid: "123".to_string(),
                name: "MyApp".to_string(),
                actions: vec![Action {
                    uuid: "456".to_string(),
                    auth: "None".to_string(),
                    scope: "Public".to_string(),
                    etag: "123".to_string(),
                }],
            }],
        };
        let result = action_exists_in_app(&cloud, "123".to_string(), "456".to_string());
        assert!(result);
    }

    #[test]
    fn test_action_does_not_exists_in_app() {
        let cloud = Cloud {
            name: "BettyBlocks".to_string(),
            applications: vec![Application {
                uuid: "123".to_string(),
                name: "MyApp".to_string(),
                actions: vec![Action {
                    uuid: "456".to_string(),
                    auth: "None".to_string(),
                    scope: "Public".to_string(),
                    etag: "123".to_string(),
                }],
            }],
        };
        let result = action_exists_in_app(&cloud, "123".to_string(), "46".to_string());
        assert!(!result);
    }

    #[test]
    fn test_application_does_not_exists_in_app() {
        let cloud = Cloud {
            name: "BettyBlocks".to_string(),
            applications: vec![Application {
                uuid: "123".to_string(),
                name: "MyApp".to_string(),
                actions: vec![Action {
                    uuid: "456".to_string(),
                    auth: "None".to_string(),
                    scope: "Public".to_string(),
                    etag: "123".to_string(),
                }],
            }],
        };
        let result = action_exists_in_app(&cloud, "13".to_string(), "46".to_string());
        assert!(!result);
    }
}
