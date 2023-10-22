slint::include_modules!();

fn main() -> Result<(), slint::PlatformError> {
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
        println!("text changed -> {}", ui.get_thetext());
    });

    ui.run()
}
