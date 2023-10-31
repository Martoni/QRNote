use image::Rgb;
use qrcode::QrCode;
use slint::Image;
use slint::Rgb8Pixel;
use slint::SharedPixelBuffer;
slint::include_modules!();

#[cfg_attr(target_arch = "wasm32",
           wasm_bindgen::prelude::wasm_bindgen(start))]
pub fn main() -> Result<(), slint::PlatformError> {
    let ui = AppWindow::new()?;

    let ui_handle = ui.as_weak();
    ui.on_text_is_edited(move || {
        let ui = ui_handle.unwrap();
        match QrCode::new(ui.get_thetext()) {
            Ok(code) => {
                let image = code.render::<Rgb<u8>>().build();
                let pixel_buffer = SharedPixelBuffer::<Rgb8Pixel>::clone_from_slice(
                    image.as_raw(),
                    image.width(),
                    image.height(),
                );
                ui.set_errormsg("Ok".into());
                ui.set_qrnote(Image::from_rgb8(pixel_buffer));
            }
            Err(e) => ui.set_errormsg(e.to_string().into()),
        }
    });

    ui.run()
}
