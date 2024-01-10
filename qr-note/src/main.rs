use image::Rgb;
use qrcode::QrCode;
use slint::Image;
use slint::Rgb8Pixel;
use slint::SharedPixelBuffer;
slint::include_modules!();

#[cfg(target_arch = "wasm32")]
use wasm_bindgen::prelude::*;


#[cfg(target_arch = "wasm32")]
#[wasm_bindgen(start)]
pub fn start() {
    main().unwrap();
}

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
                let s = String::from(ui.get_thetext());
                ui.set_charcount(s.chars().count().try_into().unwrap());
                ui.set_errormsg("".into());
                ui.set_qrnote(Image::from_rgb8(pixel_buffer));

            }
            Err(e) => ui.set_errormsg(e.to_string().into()),
        }
    });

    ui.run()
}
