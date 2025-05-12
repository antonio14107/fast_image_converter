/*
This project is inspired by the open-source repository:
https://github.com/briannelson95/rust-image-converter

We adapted it to include:
- A graphical user interface (using egui)
- Parallel processing with rayon
- Auto-detection of common image formats (case-insensitive)
- Robust error handling and progress reporting
- Quit button with graceful shutdown
- Improved, visually polished UI
- Modular code structure
- Compression level control
*/

mod types;
mod converter;
mod gui;

use types::FastImageConverter;
use eframe::egui;

fn main() -> eframe::Result<()> {
    let options = eframe::NativeOptions {
        initial_window_size: Some(egui::vec2(600.0, 400.0)),
        ..Default::default()
    };
    
    eframe::run_native(
        "Fast Image Converter",
        options,
        Box::new(|_cc| Box::new(FastImageConverter::default())),
    )
}
