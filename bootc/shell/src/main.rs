// SPDX-License-Identifier: Apache-2.0

slint::include_modules!();

fn main() -> Result<(), slint::PlatformError> {
    let window = MainWindow::new()?;

    window.run()
}
