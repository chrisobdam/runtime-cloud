wit_bindgen::generate!({ generate_all });
use exports::bettyblocks::runtime_cloud::meta_artefact::Guest;
use exports::wasi::http::incoming_handler::Guest as IncomingHandler;
use wasi::http::types::*;
use wasi::logging::logging::{log, Level};
mod artefact;

struct Component;

export!(Component);

impl Guest for Component {
    fn validate(app_uuid: String, action_uuid: String) -> Result<bool, String> {
        //for now we always first store the artefact
        artefact::write_artefact(&"".to_string()).expect("error");

        match artefact::validate(app_uuid, action_uuid) {
            Ok(result) => Ok(result),
            Err(err) => Err(err),
        }
    }
}

impl IncomingHandler for Component {
    //this handle implements the /artefact-webhook endpoint
    fn handle(_request: IncomingRequest, response_out: ResponseOutparam) {
        let result = artefact::write_artefact(&"".to_string()).expect("error");
        log(Level::Info, "BG", &format!("Artefact: {}", result));

        let response = OutgoingResponse::new(Fields::new());
        response.set_status_code(200).unwrap();
        let response_body = response.body().unwrap();
        ResponseOutparam::set(response_out, Ok(response));
        response_body
            .write()
            .unwrap()
            .blocking_write_and_flush(b"Hello from Rust!\n")
            .unwrap();
        OutgoingBody::finish(response_body, None).expect("failed to finish response body");
    }
}
