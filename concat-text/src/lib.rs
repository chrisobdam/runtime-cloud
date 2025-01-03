wit_bindgen::generate!({ generate_all });
use exports::bettyblocks::runtime_cloud::concat_text::Guest;

struct ConcatText;

impl Guest for ConcatText {
    fn execute(a: String, b: String) -> String {
        format!("{} {}", a, b)
    }
}

export!(ConcatText);
