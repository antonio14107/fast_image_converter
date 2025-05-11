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
    let compression_level = app.compression_level;
    
    let processed = Arc::new(Mutex::new(0));
    let log_messages = Arc::new(Mutex::new(Vec::new()));

    paths.par_iter().for_each(|path| {
        if let Ok(img) = image::open(path) {
            let name = path.file_stem().unwrap().to_str().unwrap();
            let out_path = Path::new(&output_dir).join(format!("{}.{}", name, format_str));
            
            let result = if format == ImageFormat::Jpeg {
                let mut output = Vec::new();
                if let Ok(_) = img.write_with_encoder(
                    image::codecs::jpeg::JpegEncoder::new_with_quality(&mut output, compression_level)
                ) {
                    match fs::write(&out_path, output) {
                        Ok(_) => Ok(()),
                        Err(e) => Err(format!("IO error: {}", e))
                    }
                } else {
                    Err("JPEG encoding failed".to_string())
                }
            } else {
                match img.save_with_format(&out_path, format) {
                    Ok(_) => Ok(()),
                    Err(e) => Err(format!("Image save error: {}", e))
                }
            };
            
            if let Err(error_msg) = result {
                let mut logs = log_messages.lock().unwrap();
                logs.push(format!("Failed to convert {:?}: {}", path, error_msg));
            }
        } else {
            let mut logs = log_messages.lock().unwrap();
            logs.push(format!("Failed to open {:?}", path));
        }
        
        let mut count = processed.lock().unwrap();
        *count += 1;
    });
    
    let final_count = *processed.lock().unwrap();
    app.progress = 1.0;
    
    let errors = log_messages.lock().unwrap();
    for err in errors.iter() {
        app.log.push(err.clone());
    }
    
    app.log.push(format!("Conversion finished! Processed {} of {} images", final_count, paths.len()));
}
