use qrcode::QrCode;
use image::Luma;
slint::include_modules!();

fn main() -> Result<(), slint::PlatformError> {

    let mut count = 0;

    /* QrCode tests */
    // Save the image.
//    image.save("qrcode.png").unwrap();
    /* end of QrCode tests */

    let ui = AppWindow::new()?;

    let ui_handle = ui.as_weak();
    ui.on_request_increase_value(move || {
        let ui = ui_handle.unwrap();
        ui.set_counter(ui.get_counter() + 1);
        println!("button pushed");
    });

    let ui_handle = ui.as_weak();
    ui.on_compute_qr_code(move || {
        let ui = ui_handle.unwrap();
        println!("{}",ui.get_thetext());
    });

    let ui_handle = ui.as_weak();
    ui.on_qr_note_edited(move || {
        let ui = ui_handle.unwrap();
//        println!("text changed -> {}", ui.get_thetext());
        let code = QrCode::new(ui.get_thetext()).unwrap();
        let image = code.render::<Luma<u8>>().build();
        image.save("qrcode.png").unwrap();
    });

    ui.run()
}
