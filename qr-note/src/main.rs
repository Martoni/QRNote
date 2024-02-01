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
                let s = String::from(ui.get_thetext());
                let charcount = s.chars().count();
                if charcount > 500 {
                    ui.set_qr_size(500);
                } else if charcount > 150 {
                    ui.set_qr_size(300);
                }
                ui.set_charcount(charcount.try_into().unwrap());
                ui.set_errormsg("".try_into().unwrap());
                let image = code.render::<Rgb<u8>>().build();
                let pixel_buffer = SharedPixelBuffer::<Rgb8Pixel>::clone_from_slice(
                    image.as_raw(),
                    image.width(),
                    image.height(),
                );
                ui.set_qrnote(Image::from_rgb8(pixel_buffer));

            }
            Err(e) => ui.set_errormsg(e.to_string().into()),
        }
    });

    ui.run()
}
