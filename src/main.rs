use wasm_bindgen::prelude::*;
use web_sys::console::*;
use web_sys::{window, Document, File, FileReader, HtmlElement, HtmlInputElement, Window};
use yew::prelude::*;

use crate::utils::set_panic_hook;

mod utils;

// When the `wee_alloc` feature is enabled, use `wee_alloc` as the global
// allocator.
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

#[wasm_bindgen]
extern "C" {
    fn alert(s: &str);
}

#[wasm_bindgen]
pub fn greet() {
    set_panic_hook();
    log_1(&"Hello, encrypt!".into());
}

#[function_component]
fn App() -> Html {
    let counter = use_state(|| 0);
    let onclick = {
        let counter = counter.clone();
        move |_| {
            let value = *counter + 1;
            counter.set(value);

            let window: Window = window().expect("should have a window in this context");
            let document: Document = window
                .document()
                .expect("window should have a document")
                .dyn_into()
                .unwrap();

            let file_input: HtmlInputElement = document
                .get_element_by_id("file")
                .expect("should have file input")
                .dyn_into()
                .unwrap();

            let file_reader: FileReader = FileReader::new().expect("should have file reader");
            file_reader.set_onloadend(|_| {
                log_1(&format!("file content: {:?}", file_reader.result()).into());
            });
            file_reader
                .read_as_text(&file_input.files().unwrap().get(0).unwrap())
                .unwrap();

            let key_input = document
                .get_element_by_id("key")
                .expect("should have key input")
                .dyn_into::<HtmlInputElement>()
                .expect("should have key input");

            let key = key_input.value();
            log_1(&format!("key: {}", key).into());
        }
    };

    html! {
        <>
        <label>
            { "Input file:" }
            <input type="file" id="file" />
        </label>

        <br/>

        <label>
            { "Encryption key:" }
            <input type="password" id="key" value="biFo9shi" />
        </label>

        <br/>

        <label style="width: 80%">
            { "URL format, use {encrypted} for encrypted string:" }
            <input type="text" id="url" style="width: 100%"
                value="https://www.google.com/?pharmaid={encrypted}" />
        </label>

        <div>
            <button {onclick}>{ "Run" }</button>
        </div>
        </>
    }
}

fn main() {
    let div = window()
        .expect("should have a window in this context")
        .document()
        .expect("window should have a document")
        .get_element_by_id("app")
        .expect("should have an element with `id=\"app\"`")
        .dyn_into::<HtmlElement>()
        .expect("should have an element with `id=\"app\"`");

    yew::Renderer::<App>::with_root(div.into()).render();
}
