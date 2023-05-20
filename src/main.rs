use std::future::Future;

use js_sys::Function;
use wasm_bindgen::prelude::*;
use wasm_bindgen_futures::spawn_local;
use web_sys::{Document, FileReader, HtmlElement, HtmlInputElement, window, Window};
use web_sys::console::*;
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

            // Create a new FileReader object and unwrap the result
            let file_reader = FileReader::new().unwrap();

// Wrap a closure that takes no arguments and returns nothing in a Box
// The closure captures the file_reader by cloning it
// The closure logs the result of reading the file as a string
            let onload_fn = Closure::wrap(Box::new(move || {
// Get the result of the file_reader and unwrap it
                let result = file_reader.clone().result().unwrap();
// Convert the result to a string and unwrap it
                let result = result.as_string().unwrap();
// Log the file content using the log_1 function
                log_1(&format!("file content: {}", result).into());
            }) as Box<dyn FnMut()>);

// Set the onload property of the file_reader to the closure reference
            file_reader.set_onload(Some(onload_fn.as_ref().unchecked_ref()));

// Get the first file from the file_input and unwrap it
            let file = file_input.files().unwrap().get(0).unwrap();
// Read the file as text and unwrap the result
            file_reader.read_as_text(&file).unwrap();

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
        <form method="post" enctype="multipart/form-data">
        <label>
            { "Input file:" }
            <input type="file" id="file" accept=".csv" />
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
        </form>
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

    // yew::Renderer::<App>::new().render();
    // yew::start_app::<App>();
    // yew::start_app_in_element(div, App::init);
}
