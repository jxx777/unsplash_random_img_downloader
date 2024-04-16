# Image Downloader CLI
___

This Rust-based project offers a command-line tool that utilizes asynchronous operations with Tokio to download images in specified resolutions. It features a simple interface that requires no API keys, making it accessible for immediate use without any overhead associated with API management.

## Features
___

- **Asynchronous Image Downloading**: Efficiently download multiple images concurrently.
- **Multiple Resolutions Supported**: Choose from Full HD, Quad HD, and 4K, available in both standard and vertical orientations.
- **Interactive User Prompts**: Step-by-step CLI prompts to specify resolution, search query, and number of images.
- **No API Key Required**: Operate without any API key, subscription, or setup overhead.

## Prerequisites
___

Ensure you have Rust installed on your system. If not, you can install it from [here](https://www.rust-lang.org/tools/install). This project uses various Rust crates, which will be automatically handled by Cargo.

## Installation
___

Clone this repository and move into the project directory:

```bash
git clone https://yourrepository.com/image-downloader-cli.git
```

```bash
cd image-downloader-cli
```

Compile the project using Cargo:

```bash
cargo build --release
```

The executable will be located in target/release.

## Usage
___
To run the application, use the following command:
```bash
cargo run --release
```

You will be prompted to choose a resolution, enter a search query, and specify the number of images to download.