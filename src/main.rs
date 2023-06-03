use std::io::BufWriter;

use aes_gcm::{
    aead::{Aead, AeadCore, KeyInit, OsRng},
    Aes128Gcm, Key,
};
use base64::{engine::general_purpose, Engine as _};
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
struct CsvInput {
    #[serde(rename = "Email")]
    email: String,
    #[serde(rename = "First Name")]
    first_name: String,
    #[serde(rename = "Date of Service")]
    date_of_service: String,
}

#[derive(Clone, Debug, Deserialize, Serialize, PartialEq)]
struct CsvOutput {
    #[serde(rename = "Email")]
    email: String,
    #[serde(rename = "First Name")]
    first_name: String,
    #[serde(rename = "Date of Service")]
    date_of_service: String,
    #[serde(rename = "URL")]
    url: String,
}

fn aes_encrypt(key: &[u8], data: &[u8]) -> String {
    let key = Key::<Aes128Gcm>::from_slice(key);
    let cipher = Aes128Gcm::new(&key);
    let nonce = Aes128Gcm::generate_nonce(&mut OsRng); // 96-bits; unique per message
    let ciphertext = cipher.encrypt(&nonce, data).unwrap();
    let payload = [nonce.as_slice(), &ciphertext].concat();
    general_purpose::URL_SAFE_NO_PAD.encode(&payload)
}

#[function_component]
fn App() -> Html {
    let counter = use_state(|| State { value: 0 });
    let onclick = {
        let counter = counter.clone();

        move |ev: MouseEvent| {
            ev.prevent_default();
            // log_1(&"Hello, encrypt, inc!".into());

            counter.set(State {
                value: counter.value + 1,
            });

            let window: Window = window().expect("should have a window in this context");

            let t1 = window.performance().unwrap().now();
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

            let url_format = document
                .get_element_by_id("url")
                .expect("should have url format")
                .dyn_into::<HtmlInputElement>()
                .unwrap()
                .value();

            if let Some(file) = file_input.files().unwrap().get(0) {
                let fr = &FileReader::new().unwrap();

                let cb = Closure::once_into_js(move |event: Event| {
                    // do nothing
                    log_1(&"File read!".into());
                    let fr = event.target().unwrap().dyn_into::<FileReader>().unwrap();
                    let input = fr.result().unwrap();
                    let input_str = input.as_string().unwrap();
                    // log_1(&input_str.clone().into());

                    let buf = Vec::new();
                    let result_file = BufWriter::new(buf);

                    let mut csv_writer = csv::Writer::from_writer(result_file);

                    csv::Reader::from_reader(input_str.as_bytes())
                        .deserialize::<CsvInput>()
                        .for_each(|row| {
                            let r = row.expect("csv row").clone();
                            // log_1(&format!("{:?}", r).into());

                            let encrypted =
                                aes_encrypt(r.date_of_service.as_bytes(), "biFo9shi".as_bytes());

                            let url = url_format.replace("{encrypted}", &encrypted);

                            csv_writer
                                .serialize(CsvOutput {
                                    email: r.email,
                                    first_name: r.first_name,
                                    date_of_service: r.date_of_service,
                                    url,
                                })
                                .unwrap();
                        });

                    csv_writer.flush().unwrap();
                    // Flush the writer and get the buffer
                    let buf = csv_writer
                        .into_inner()
                        .expect("failed to get buffer")
                        .into_inner();

                    // log_1(&JsValue::from(String::from_utf8(buf.unwrap()).unwrap()));

                    let t2 = window.performance().unwrap().now();
                    log_1(&format!("Time: {} ms", t2 - t1).into());
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

        <textarea id="result" style="width: 100%; height: 500px"></textarea>

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
