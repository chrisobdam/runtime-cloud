use data_encoding::BASE64_NOPAD;
use std::io::Read;
use wasi::logging::logging::{log, Level};
use wasmcloud_component::http;
wit_bindgen::generate!({ generate_all });
use http::Method;
mod graphql;

struct Component;

http::export!(Component);

fn get_app_uuid_from_token(token: &str) -> Result<String, String> {
    if token.is_empty() {
        return Err("No Authorization header found".to_string());
    }

    let parts: Vec<&str> = token.split('.').collect();

    if parts.len() != 3 {
        return Err("Invalid JWT format".to_string());
    }

    // needs to be NOPAD
    let payload = match BASE64_NOPAD.decode(parts[1].as_bytes()) {
        Ok(decoded) => decoded,
        Err(e) => return Err(format!("Failed to decode payload: {:?}", e)),
    };

    let payload_str = String::from_utf8(payload).expect("Invalid UTF-8 sequence in payload");
    let payload_json: serde_json::Value = serde_json::from_str(&payload_str).unwrap();
    match payload_json["app_uuid"].as_str() {
        Some(app_uuid) => Ok(app_uuid.to_string()),
        None => Err("app_uuid not found in token".to_string()),
    }
}

fn incoming_body_to_string(mut body: http::IncomingBody) -> String {
    let mut buf = vec![];
    body.read_to_end(&mut buf)
        .expect("should have read incoming buffer");

    let body_text = String::from_utf8(buf).expect("no valid UTF8");
    body_text
}

fn headers_to_authorization(headers: &http::HeaderMap) -> String {
    headers
        .get("Authorization")
        .and_then(|value| value.to_str().ok())
        .map(|value| value.to_string())
        .unwrap_or_else(|| "".to_string())
}

fn set_link_name(name: &str) {
    let interface =
        wasmcloud::bus::lattice::CallTargetInterface::new("bettyblocks", "runtime-cloud", name);
    wasmcloud::bus::lattice::set_link_name(name, vec![interface]);
}

impl http::Server for Component {
    fn handle(
        _request: http::IncomingRequest,
    ) -> http::Result<http::Response<impl http::OutgoingBody>> {
        let (_parts, body) = _request.into_parts();

        let body_text = incoming_body_to_string(body);

        let authorization_header = headers_to_authorization(&_parts.headers);

        // if authorization_header == "" {
        //     return Ok(http::Response::new("Authorization header not found"));
        // }

        // let app_uuid =
        //     get_app_uuid_from_token(&authorization_header).unwrap_or_else(|_| "".to_string());

        let action_uuid = "456".to_string();

        // let interface = wasmcloud::bus::lattice::CallTargetInterface::new(
        //     "bettyblocks",
        //     "runtime-cloud",
        //     "meta-artefact",
        // );
        // wasmcloud::bus::lattice::set_link_name("cloud-artefact", vec![interface]);
        // // log(Level::Info, "", &format!("app_uuid: {:?}", &app_uuid));

        // let result = bettyblocks::runtime_cloud::meta_artefact::validate(&app_uuid, &action_uuid);
        let valid = bettyblocks::runtime_cloud::meta_artefact::validate;

        match _parts.method {
            Method::POST => {
                // set_link_name("action-runner");
                // let str = bettyblocks::runtime_cloud::action_runner::execute();
                match get_app_uuid_from_token(&authorization_header) {
                    Ok(app_uuid) => {
                        let interface = wasmcloud::bus::lattice::CallTargetInterface::new(
                            "bettyblocks",
                            "runtime-cloud",
                            "meta-artefact",
                        );
                        wasmcloud::bus::lattice::set_link_name("cloud-artefact", vec![interface]);
                        match valid(&app_uuid, &action_uuid) {
                            Ok(_) => {
                                return Ok(http::Response::new(format!(
                                    "{} {}",
                                    body_text, "Validated"
                                )));
                            }
                            Err(e) => {
                                return Ok(http::Response::new(format!("{} {}", body_text, e)));
                            }
                        }
                    }
                    Err(e) => {
                        return Ok(http::Response::new(format!("{} {}", body_text, e)));
                    }
                }
            }
            _ => {
                // Handle non POST request logic here
                return Ok(http::Response::new(format!(
                    "Only POST requests are allowed"
                )));
            }
        }
    }
}

#[cfg(test)]
mod tests {
    // use wasi::http::outgoing_handler::handle;

    use super::*;

    #[test]
    fn test_get_application_id_from_token_success() {
        let token = "eyJhbGciOiJIUzUxMiIsInR5cCI6IkpXVCJ9.eyJhcHBfdXVpZCI6IjY5M2IyMmU5ODNmYjQ2YWZhNGViMzUzZDgyZWNlNGJiIiwiYXVkIjoiSm9rZW4iLCJhdXRoX3Byb2ZpbGUiOiI1YmY5ZWJhMzQ2MzY0OTVkODBlZDVhNzkwY2EzOTA3NyIsImNhc190b2tlbiI6ImQ2NTI1ODU5NjRlY2ZkNTliZDczOGJiMzNmNWE0MjFjZTg1YzQ5M2UiLCJleHAiOjE3MzYwODA1NDIsImlhdCI6MTczNjA3MzM0MiwiaXNzIjoiSm9rZW4iLCJqdGkiOiIzMGJzYjBkb2lidHUycHJ0NTAwMDcwZDMiLCJsb2NhbGUiOm51bGwsIm5iZiI6MTczNjA3MzM0Miwicm9sZXMiOlsxXSwidXNlcl9pZCI6MX0.qnlhrIcCbVSf5szKBJSnjsVLz_b8Cem-Bfwe6u-_921UYS9qRGJGpsZ9Sr7aAGz1NwC78eXT0GTuAz4fL28k_A";
        let result = get_app_uuid_from_token(token);
        assert_eq!(
            result.unwrap(),
            "693b22e983fb46afa4eb353d82ece4bb".to_string()
        );
    }

    #[test]
    fn test_get_application_id_from_token_invalid() {
        let token = "eyJhbGciOiJIUzUxMiIsInR5cCI6IkpXVCJ9eyJhcHBfdXVpZCI6IjY5M2IyMmU5ODNmYjQ2YWZhNGViMzUzZDgyZWNlNGJiIiwiYXVkIjoiSm9rZW4iLCJhdXRoX3Byb2ZpbGUiOiI1YmY5ZWJhMzQ2MzY0OTVkODBlZDVhNzkwY2EzOTA3NyIsImNhc190b2tlbiI6ImQ2NTI1ODU5NjRlY2ZkNTliZDczOGJiMzNmNWE0MjFjZTg1YzQ5M2UiLCJleHAiOjE3MzYwODA1NDIsImlhdCI6MTczNjA3MzM0MiwiaXNzIjoiSm9rZW4iLCJqdGkiOiIzMGJzYjBkb2lidHUycHJ0NTAwMDcwZDMiLCJsb2NhbGUiOm51bGwsIm5iZiI6MTczNjA3MzM0Miwicm9sZXMiOlsxXSwidXNlcl9pZCI6MX0.qnlhrIcCbVSf5szKBJSnjsVLz_b8Cem-Bfwe6u-_921UYS9qRGJGpsZ9Sr7aAGz1NwC78eXT0GTuAz4fL28k_A";
        let result = get_app_uuid_from_token(token);
        assert_eq!(result.err(), Some("Invalid JWT format".to_string()));
    }

    #[test]
    fn test_get_application_id_from_empty_token_invalid() {
        let token = "";
        let result = get_app_uuid_from_token(token);
        assert_eq!(result.err(), Some("Invalid JWT format".to_string()));
    }

    #[test]
    fn test_get_app_uuid_with_json_but_no_app_uuid() {
        let token = "eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9.eyJrZXkiOiJ2YWx1ZSIsImlhdCI6MTczNjYwNTA1M30.fBzbUcrAomhFLFF9HGodYIIPBsU-gzsUbQBSAcfa0UE";
        let result = get_app_uuid_from_token(token);
        assert_eq!(
            result.err(),
            Some("app_uuid not found in token".to_string())
        );
    }
}
