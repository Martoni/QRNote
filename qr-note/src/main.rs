use qrcode::QrCode;
use image::Luma;
use std::path::Path;
slint::include_modules!();

fn main() -> Result<(), slint::PlatformError> {
    let ui = AppWindow::new()?;

    let ui_handle = ui.as_weak();
    ui.on_request_increase_value(move || {
        let ui = ui_handle.unwrap();
        ui.set_counter(ui.get_counter() + 1);
        println!("button pushed");
    });
    let mut count = 0;
    let ui_handle = ui.as_weak();
    ui.on_compute_qr_code(move || {
        let ui = ui_handle.unwrap();
        let code = QrCode::new(ui.get_thetext()).unwrap();
        let image = code.render::<Luma<u8>>().build();
//        fs::remove_file("ui/icons/qrcode.png").unwrap();
        image.save("ui/icons/qrcode.png").unwrap();
        use slint::Image;
        let qrimage = Image::load_from_path(Path::new("ui/icons/qrcode.png")).unwrap();
        ui.set_qrnote(qrimage);
        println!("count {}", count);
        count = count + 1;
    });

    ui.run()
}
