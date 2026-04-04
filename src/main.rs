slint::include_modules!();
fn main() -> Result<(), slint::platform::PlatformError> {
    let ui = MainWindow::new().unwrap();

    ui.run()
}