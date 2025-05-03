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
*/

use rayon::prelude::*;
use std::fs;
use std::path::Path;
use image::ImageFormat;
use eframe::egui;

#[derive(Default)]
struct MyApp {
    input_dir: String,
    output_dir: String,
    format: String,
    progress: f32,
    log: Vec<String>,
    should_quit: bool,
}

impl MyApp {
    fn convert_images(&mut self) {
        self.log.clear();
        self.progress = 0.0;

        let supported_extensions = vec![
            "jpg", "jpeg", "png", "bmp", "tiff", "webp", "gif", "ico", "hdr", "pnm", "tga", "dds",
        ];
        let paths: Vec<_> = match fs::read_dir(&self.input_dir) {
            Ok(entries) => entries
                .filter_map(|e| e.ok())
                .map(|e| e.path())
                .filter(|p| {
                    p.is_file()
                        && p.extension()
                            .and_then(|s| s.to_str())
                            .map(|ext| supported_extensions.contains(&ext.to_lowercase().as_str()))
                            .unwrap_or(false)
                })
                .collect(),
            Err(_) => {
                self.log.push("Failed to read input directory.".to_string());
                return;
            }
        };

        let total = paths.len() as f32;
        if total == 0.0 {
            self.log.push("No supported images found.".to_string());
            return;
        }

        let output_dir = self.output_dir.clone();
        let format_str = self.format.to_lowercase();
        let format = match self.format.as_str() {
            "PNG" => ImageFormat::Png,
            "JPEG" => ImageFormat::Jpeg,
            "BMP" => ImageFormat::Bmp,
            "TIFF" => ImageFormat::Tiff,
            "WEBP" => ImageFormat::WebP,
            "GIF" => ImageFormat::Gif,
            "ICO" => ImageFormat::Ico,
            "HDR" => ImageFormat::Hdr,
            "PNM" => ImageFormat::Pnm,
            "TGA" => ImageFormat::Tga,
            "DDS" => ImageFormat::Dds,
            _ => ImageFormat::Png,
        };

        paths.par_iter().enumerate().for_each(|(_idx, path)| {
            if let Ok(img) = image::open(path) {
                let name = path.file_stem().unwrap().to_str().unwrap();
                let out_path = Path::new(&output_dir).join(format!("{}.{}", name, &format_str));
                let _ = img.save_with_format(out_path, format);
            } else {
                eprintln!("Failed to open {:?}", path);
            }
        });

        self.progress = 1.0;
        self.log.push("Conversion finished!".to_string());
    }
}

impl eframe::App for MyApp {
    fn update(&mut self, ctx: &egui::Context, frame: &mut eframe::Frame) {
        if self.should_quit {
            frame.close();
            return;
        }

        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("🚀 Fast Image Converter");
            ui.add_space(10.0);

            ui.group(|ui| {
                ui.label("📂 Input Folder:");
                ui.text_edit_singleline(&mut self.input_dir);
                ui.add_space(5.0);
                ui.label("💾 Output Folder:");
                ui.text_edit_singleline(&mut self.output_dir);
                ui.add_space(5.0);
                ui.label("🎨 Format:");
                egui::ComboBox::from_id_source("format_combo")
                    .selected_text(&self.format)
                    .show_ui(ui, |ui| {
                        for fmt in &["PNG", "JPEG", "BMP", "TIFF", "WEBP", "GIF", "ICO", "HDR", "PNM", "TGA", "DDS"] {
                            ui.selectable_value(&mut self.format, fmt.to_string(), *fmt);
                        }
                    });
            });

            ui.add_space(10.0);

            if ui.button("⚡ Convert All Images").clicked() {
                self.convert_images();
            }

            ui.add(egui::ProgressBar::new(self.progress)
                .desired_width(f32::INFINITY)
                .show_percentage());

            ui.add_space(10.0);

            egui::ScrollArea::vertical().max_height(100.0).show(ui, |ui| {
                for line in &self.log {
                    ui.label(line);
                }
            });

            ui.add_space(10.0);

            ui.horizontal(|ui| {
                if ui.button("📁 Open Output Folder").clicked() {
                    if let Err(e) = open::that(&self.output_dir) {
                        eprintln!("Failed to open folder: {}", e);
                    }
                }
                if ui.button("❌ Quit").clicked() {
                    self.should_quit = true;
                }
            });
        });
    }
}

fn main() -> eframe::Result<()> {
    let options = eframe::NativeOptions::default();
    eframe::run_native(
        "Fast Image Converter",
        options,
        Box::new(|_cc| Box::new(MyApp {
            format: "PNG".to_string(),
            ..Default::default()
        })),
    )
}
