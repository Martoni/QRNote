use qrcode::QrCode;
use image::Luma;
use image::Rgb;
use slint::SharedPixelBuffer;
use slint::Rgb8Pixel;
use slint::Image;
slint::include_modules!();

fn main() -> Result<(), slint::PlatformError> {
    let ui = AppWindow::new()?;

    let ui_handle = ui.as_weak();
    ui.on_text_is_edited(move || {
        let ui = ui_handle.unwrap();
        let code = QrCode::new(ui.get_thetext()).unwrap();
        let image = code.render::<Rgb<u8>>().build();
        let pixel_buffer = SharedPixelBuffer::<Rgb8Pixel>::clone_from_slice(
            image.as_raw(),
            image.width(),
            image.height());
        ui.set_qrnote(Image::from_rgb8(pixel_buffer));
    });
 
    ui.run()
}
