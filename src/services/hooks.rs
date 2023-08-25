use wasm_bindgen_futures::spawn_local;
use yew::prelude::*;
use yew::suspense::SuspensionResult;
use crate::services::handler::FileHandler;

#[hook]
pub fn use_file(path: String) -> SuspensionResult<String> {
    let state = use_state_eq(|| Some("not found".to_string()));
    let md = {
        let state_handle_clone = state.clone();
        spawn_local(async move {
            let result = FileHandler::retrieve(&path).await.unwrap();
            state_handle_clone.set(Some(result));
        });
        state.as_ref().cloned()
    };
    
    Ok(md.unwrap())
}
