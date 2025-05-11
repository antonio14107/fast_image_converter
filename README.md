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
