use serde_derive::{Deserialize, Serialize};
use wasm_bindgen::prelude::*;
use web_sys::console::*;
use web_sys::{window, Document, FileReader, HtmlElement, HtmlInputElement, Window};
use yew::prelude::*;

use crate::utils::set_panic_hook;

mod utils;

// When the `wee_alloc` feature is enabled, use `wee_alloc` as the global
// allocator.
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

#[derive(Clone, Debug, PartialEq)]
struct State {
    value: i64,
}

#[derive(Clone, Debug, Deserialize, Serialize, PartialEq)]
struct CsvRow {
    #[serde(rename = "Email")]
    email: String,
    #[serde(rename = "First Name")]
    first_name: String,
    #[serde(rename = "Date of Service")]
    date_of_service: String,
}

#[function_component]
fn App() -> Html {
    let counter = use_state(|| State { value: 0 });
    let onclick = {
        let counter = counter.clone();

        move |ev: MouseEvent| {
            ev.prevent_default();
            log_1(&"Hello, encrypt, inc!".into());

            counter.set(State {
                value: counter.value + 1,
            });

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

            if let Some(file) = file_input.files().unwrap().get(0) {
                let fr = &FileReader::new().unwrap();

                let cb = Closure::once_into_js(move |event: Event| {
                    // do nothing
                    log_1(&"Hello, encrypt, onloadend!".into());
                    let fr = event.target().unwrap().dyn_into::<FileReader>().unwrap();
                    let result = fr.result().unwrap();
                    let result_str = result.as_string().unwrap();
                    log_1(&result_str.clone().into());

                    csv::Reader::from_reader(result_str.as_bytes())
                        .deserialize::<CsvRow>()
                        .for_each(|row| {
                            log_1(&format!("{:?}", row).into());
                        });
                });

                fr.set_onloadend(Some(cb.unchecked_ref()));
                fr.read_as_text(&file).unwrap();
            } else {
                log_1(&"No file selected".into());
            }
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
        <p>
            { "Counter: " } { (*counter).value }
        </p>
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
    set_panic_hook();

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
