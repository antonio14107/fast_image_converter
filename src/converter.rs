use crate::types::FastImageConverter;
use std::fs;
use std::path::Path;
use image::ImageFormat;
use rayon::prelude::*;
use std::sync::{Arc, Mutex};

pub fn convert_images(app: &mut FastImageConverter) {
    app.log.clear();
    app.progress = 0.0;

    let supported_extensions = vec![
        "jpg", "jpeg", "png", "bmp", "tiff", "webp", "gif", "ico", "hdr", "pnm", "tga", "dds",
    ];

    let paths: Vec<_> = match fs::read_dir(&app.input_dir) {
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
            app.log.push("Failed to read input directory.".to_string());
            return;
        }
    };

    let total = paths.len() as f32;
    if total == 0.0 {
        app.log.push("No supported images found.".to_string());
        return;
    }

    let output_dir = app.output_dir.clone();
    let format_str = app.format_extension();
    let format = app.get_image_format();
    let _compression_level = app.compression_level;

    let processed = Arc::new(Mutex::new(0));
    let log_messages = Arc::new(Mutex::new(Vec::new()));
    let success_count = Arc::new(Mutex::new(0));

    paths.par_iter().for_each(|path| {
        let stem = path.file_stem().unwrap().to_str().unwrap();
        let ext = path.extension().unwrap().to_str().unwrap().to_lowercase();
        let unique_name = format!("{}_{}.{}", stem, ext, format_str);
        let out_path = Path::new(&output_dir).join(&unique_name);

        println!("Converting: {} -> {}", path.display(), out_path.display());

        let result = match image::open(path) {
            Ok(img) => {
                if format == ImageFormat::Jpeg {
                    let rgb_img = img.to_rgb8();
                    match rgb_img.save_with_format(&out_path, format) {
                        Ok(_) => {
                            let mut success = success_count.lock().unwrap();
                            *success += 1;
                            Ok(())
                        }
                        Err(e) => Err(format!("Failed to save JPEG: {}", e)),
                    }
                } else {
                    match img.save_with_format(&out_path, format) {
                        Ok(_) => {
                            let mut success = success_count.lock().unwrap();
                            *success += 1;
                            Ok(())
                        }
                        Err(e) => Err(format!("Failed to save image: {}", e)),
                    }
                }
            }
            Err(e) => Err(format!("Failed to open image {:?}: {}", path, e)),
        };

        if let Err(error_msg) = result {
            println!("Error processing {}: {}", path.display(), error_msg);
            let mut logs = log_messages.lock().unwrap();
            logs.push(format!("Error processing {:?}: {}", path, error_msg));
        }

        let mut count = processed.lock().unwrap();
        *count += 1;
    });

    let final_processed = *processed.lock().unwrap();
    let successful = *success_count.lock().unwrap();
    app.progress = 1.0;

    let errors = log_messages.lock().unwrap();
    for err in errors.iter() {
        app.log.push(err.clone());
    }

    app.log.push(format!(
        "Conversion finished! Successfully converted {} of {} images",
        successful, final_processed
    ));
}
