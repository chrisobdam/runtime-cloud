wit_bindgen::generate!({ generate_all });
use crate::bettyblocks::runtime_cloud::system_info::Kind;
use exports::bettyblocks::runtime_cloud::action::Guest;

struct Action;

impl Guest for Action {
    fn execute() -> String {
        let s = crate::bettyblocks::runtime_cloud::system_info::request_info(Kind::Os);
        let str = format!("action b {}", s);
        str
    }
}

export!(Action);
