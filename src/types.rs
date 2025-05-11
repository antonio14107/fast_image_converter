use image::ImageFormat;

pub struct FastImageConverter {
    pub input_dir: String,
    pub output_dir: String,
    pub format: String,
    pub compression_level: u8,
    pub progress: f32,
    pub log: Vec<String>,
    pub should_quit: bool,
}

impl Default for FastImageConverter {
    fn default() -> Self {
        Self {
            input_dir: String::new(),
            output_dir: String::new(),
            format: "PNG".to_string(),
            compression_level: 80,
            progress: 0.0,
            log: Vec::new(),
            should_quit: false,
        }
    }
}

impl FastImageConverter {
    pub fn get_image_format(&self) -> ImageFormat {
        match self.format.as_str() {
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
        }
    }
    
    pub fn format_extension(&self) -> &str {
        match self.format.as_str() {
            "PNG" => "png",
            "JPEG" => "jpg",
            "BMP" => "bmp",
            "TIFF" => "tiff",
            "WEBP" => "webp",
            "GIF" => "gif",
            "ICO" => "ico",
            "HDR" => "hdr",
            "PNM" => "pnm",
            "TGA" => "tga",
            "DDS" => "dds",
            _ => "png",
        }
    }
}
