use std::path::{Path, PathBuf};
use espup::{InstallOpts, install};
use fltk::{app, button::Button, frame::Frame, prelude::*, window::Window};
mod ui;

fn main() {
    let app = app::App::default().with_scheme(app::Scheme::Gtk);
    let mut ui = ui::UserInterface::make_window();

    ui.default_host_text_input.set_value("x86_64-pc-windows-msvc");
    ui.esp_idf_version_text_input.set_value("v4.2");
    ui.export_file_text_input.set_value("esp-export.ps1");
    ui.extra_crates_text_input.set_value("cargo-generate");
    ui.llvm_version_text_input.set_value("15");
    ui.nightly_version_text_input.set_value("nightly-2021-03-01");
    ui.toolchain_version_text_input.set_value("1.65.0.1");

    ui.install_button.set_callback(move |_| {
        let opts = InstallOpts {
            default_host: Some(ui.default_host_text_input.value()),
            esp_idf_version: Some(ui.esp_idf_version_text_input.value()),
            export_file: None,
            extra_crates: None,
            llvm_version: ui.llvm_version_text_input.value(),
            log_level: "".to_string(),
            nightly_version: ui.nightly_version_text_input.value(),
            profile_minimal: false,
            targets: Default::default(),
            toolchain_version: Some(ui.toolchain_version_text_input.value()),
        };
        install(opts);
        // ui.install_button.set_label("Installation done")
        println!("Installation done");
    });

    app.run().unwrap();
}
