use wasm_bindgen::prelude::*;
use wasm_bindgen_futures::JsFuture;
use web_sys::Response;

pub struct FileHandler;

impl FileHandler {
    /// Retrieves a file using `fetch` via WebAssembly
    pub async fn retrieve(path: &String) -> Option<String> {
        let window = web_sys::window().expect("Unable to retrieve window");
        let resp_value = JsFuture::from(window.fetch_with_str(path)).await.expect("Unable to retrieve file");

        // `resp_value` is a `Response` object.
        assert!(resp_value.is_instance_of::<Response>());
        let resp: Response = resp_value.dyn_into().expect("Unable to process response");

        // Convert this other `Promise` into a rust `Future`.
        let text = JsFuture::from(resp.text().expect("Could not get text")).await.expect("Unable to process Promise to Future");

        text.as_string()
    }
}
