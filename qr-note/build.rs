#[cfg_attr(target_arch = "wasm32",
           wasm_bindgen::prelude::wasm_bindgen(start))]
fn main() {
    slint_build::compile("ui/appwindow.slint").unwrap();
}
