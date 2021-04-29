use std::f64;
use wasm_bindgen::prelude::*;
use web_sys::console;
use wasm_bindgen::JsCast;

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
#[wasm_bindgen]
pub fn draw() {
    let document = web_sys::window().unwrap().document().unwrap();
    let canvas = document.get_element_by_id("canvas").unwrap();
    let canvas: web_sys::HtmlCanvasElement = canvas.dyn_into::<web_sys::HtmlCanvasElement>().map_err(|_| ()).unwrap();
    let context = canvas
        .get_context("2d")
        .unwrap()
        .unwrap()
        .dyn_into::<web_sys::CanvasRenderingContext2d>()
        .unwrap();

    context.begin_path();
    context.arc(75.0, 75.0, 50.0, 0.0, f64::consts::PI * 2.0).unwrap();
    context.stroke();
}

mod fibonacci;
#[wasm_bindgen]
pub fn print_fibo(n: i32) {
    for i in 0..n {
        console::log_1(&format!("{}", fibonacci::fibonacci(i)).into());
    }
}