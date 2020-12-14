
use wasm_bindgen::prelude::*;

// wasm-bindgen will automatically take care of including this script
#[wasm_bindgen(module = "/src/show_chart.js")]
extern "C" {

    #[wasm_bindgen(js_name = "load_dashboard")]
    pub fn load_dashboard(dashboard: JsValue);
}