use wasm_bindgen::prelude::*;

#[wasm_bindgen(start)]
pub fn start() -> Result<(), JsValue> {
    console_error_panic_hook::set_once();
    console_log::init_with_level(log::Level::Info).unwrap();

    let window = web_sys::window().expect("window should exist");
    let document = window.document().expect("document should exist");
    let body = document.body().expect("body should exist");

    let canvas = document.create_element("canvas")?;
    let canvas: web_sys::HtmlCanvasElement = canvas.dyn_into()?;
    canvas.set_width(1280);
    canvas.set_height(720);
    canvas.set_attribute("id", "fn-canvas")?;
    body.append_child(&canvas)?;

    log::info!("Fortnite-Rust web scaffold booted; actual game logic still needs browser-safe renderer/input/runtime ports.");
    Ok(())
}
