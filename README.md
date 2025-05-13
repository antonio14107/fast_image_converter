# Image Converter

A fast and efficient image converter application built with Rust, featuring a simple GUI interface and parallel processing.

> Inspired by [briannelson95/rust-image-converter](https://github.com/briannelson95/rust-image-converter), with major enhancements.

## Features

- Convert multiple images across formats (PNG, JPEG, BMP, TIFF, WEBP, GIF, ICO, HDR, PNM, TGA, DDS)
- Automatically detects supported image files (case-insensitive)
- Real-time progress bar
- High-performance parallel processing via Rayon
- Clean graphical interface using `egui`
- Drag-and-drop folder selection with `rfd`
- JPEG quality control slider
- Robust error handling with logs
- Graceful quit and safe shutdown

## Installation

Ensure [Rust](https://www.rust-lang.org/tools/install) installed.

This project uses:
- [`eframe`](https://crates.io/crates/eframe) – for GUI with `egui`
- [`image`](https://crates.io/crates/image) – for image decoding/encoding
- [`rayon`](https://crates.io/crates/rayon) – for parallel image processing
- [`rfd`](https://crates.io/crates/rfd) – for file/folder pickers
- [`open`](https://crates.io/crates/open) – to open output folder in system file explorer

Install them via Cargo automatically.

## Getting Started

### Clone the repository:

```bash
git clone https://github.com/yourusername/fast-image-converter.git
cd fast-image-converter
```

### Build and run:
```bash
cargo build
cargo run
```

## Demo

### Video
[![Watch the demo](https://img.youtube.com/vi/y-Ihx1XaWDc/0.jpg)](https://youtu.be/y-Ihx1XaWDc)

### Startup
<img width="602" alt="Screenshot 2025-05-12 at 6 02 17 PM" src="https://github.com/user-attachments/assets/95df01eb-67cb-4201-a038-ba5060e29b44" />

### Folder Selection
<img width="801" alt="Screenshot 2025-05-12 at 7 05 41 PM" src="https://github.com/user-attachments/assets/b41f31f3-d00e-4374-a03a-60104b7fcd81" />

### Format Selector
<img width="601" alt="Screenshot 2025-05-12 at 7 07 14 PM" src="https://github.com/user-attachments/assets/9ef0e458-96f4-4434-aae3-80a64120672a" />

### Unsupported File Type
<img width="599" alt="Screenshot 2025-05-12 at 7 08 23 PM" src="https://github.com/user-attachments/assets/105b0875-7209-4afa-9c03-8f9d9a9e1b4c" />

### JPEG Slider
<img width="600" alt="Screenshot 2025-05-12 at 8 00 09 PM" src="https://github.com/user-attachments/assets/f13de876-5bd2-4d16-b75e-f007c4401109" />

### Successful Conversion
<img width="601" alt="Screenshot 2025-05-12 at 8 01 53 PM" src="https://github.com/user-attachments/assets/ee78bbc5-9fc5-4188-a3c3-9d53aaffbc8a" />

### Open Output Folder
<img width="918" alt="Screenshot 2025-05-12 at 8 02 51 PM" src="https://github.com/user-attachments/assets/5de0912b-8fa9-4c26-a1e4-37a60a9c39c9" />
