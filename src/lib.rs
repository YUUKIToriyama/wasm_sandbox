use wasm_bindgen::prelude::*;
use web_sys::console;

#[wasm_bindgen]
pub fn say_hello(s: &str) {
    console::log_1(&format!("Hello, {}", &s).into());
}
#[wasm_bindgen]
pub fn display() -> Result<(), JsValue> {
    let window = web_sys::window().expect("グローバルなwindowオブジェクトが存在しません");
    let document = window.document().expect("documentを取得できません");
    let body = document.body().expect("bodyが取得できません");

    let val = document.create_element("p")?;
    val.set_text_content(Some("Hello world from Rust!"));
    body.append_child(&val)?;

    Ok(())
}