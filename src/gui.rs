use eframe::egui;
use crate::types::FastImageConverter;
use crate::converter;

impl eframe::App for FastImageConverter {
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
                ui.horizontal(|ui| {
                    ui.text_edit_singleline(&mut self.input_dir);
                    if ui.button("Browse...").clicked() {
                        if let Some(path) = rfd::FileDialog::new().pick_folder() {
                            self.input_dir = path.display().to_string();
                        }
                    }
                });
                
                ui.add_space(5.0);
                
                ui.label("💾 Output Folder:");
                ui.horizontal(|ui| {
                    ui.text_edit_singleline(&mut self.output_dir);
                    if ui.button("Browse...").clicked() {
                        if let Some(path) = rfd::FileDialog::new().pick_folder() {
                            self.output_dir = path.display().to_string();
                        }
                    }
                });
                
                ui.add_space(5.0);
                
                ui.horizontal(|ui| {
                    ui.label("🎨 Format:");
                    egui::ComboBox::from_id_source("format_combo")
                        .selected_text(&self.format)
                        .show_ui(ui, |ui| {
                            for fmt in &["PNG", "JPEG", "BMP", "TIFF", "WEBP", "GIF", "ICO", "HDR", "PNM", "TGA", "DDS"] {
                                ui.selectable_value(&mut self.format, fmt.to_string(), *fmt);
                            }
                        });
                });

                // The format comparison needs to be case insensitive
                if self.format.to_uppercase() == "JPEG" {
                    ui.horizontal(|ui| {
                        ui.label("Quality:");
                        ui.add(egui::Slider::new(&mut self.compression_level, 1..=100));
                        ui.label(format!("{}%", self.compression_level));
                    });
                }
            });

            ui.add_space(10.0);

            ui.horizontal(|ui| {
                if ui.button("⚡ Convert All Images").clicked() {
                    converter::convert_images(self);
                }
                
                if ui.button("📁 Open Output Folder").clicked() {
                    if !self.output_dir.is_empty() {
                        if let Err(e) = open::that(&self.output_dir) {
                            self.log.push(format!("Failed to open folder: {}", e));
                        }
                    } else {
                        self.log.push("No output folder specified.".to_string());
                    }
                }
                
                if ui.button("❌ Quit").clicked() {
                    self.should_quit = true;
                }
            });

            ui.add_space(10.0);

            ui.add(egui::ProgressBar::new(self.progress)
                .desired_width(f32::INFINITY)
                .show_percentage());

            ui.add_space(10.0);

            ui.group(|ui| {
                ui.label("📝 Log");
                egui::ScrollArea::vertical().max_height(120.0).show(ui, |ui| {
                    for line in &self.log {
                        ui.label(line);
                    }
                });
            });
        });
    }
}
