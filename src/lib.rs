mod app;
pub mod game_box;
pub mod grid;

use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub fn run_app() -> Result<(), JsValue> {
  wasm_logger::init(wasm_logger::Config::default());
  yew::start_app::<app::App>();
  Ok(())
}
