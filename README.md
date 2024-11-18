# HDR Image Merger 🌄

## Summary of Project 📖

The **HDR Image Merger** is a web application designed to merge multiple images taken with different exposure levels into a single High Dynamic Range (HDR) image. This app leverages WebAssembly (Wasm) and the `wasm-bindgen` library to perform image processing in the browser, providing a seamless and efficient user experience. Users can drag and drop their exposure-bracketed images to quickly create a beautifully merged HDR image.

## How to Use 🚀

1. **Clone the Repository**:
   ```bash
   git clone https://github.com/harperreed/hdr-web-app.git
   cd hdr-web-app
   ```

2. **Install Dependencies**:
   Make sure you have [Rust](https://www.rust-lang.org/tools/install) and [Cargo](https://doc.rust-lang.org/cargo/getting-started/installation.html) installed. Next, navigate to the project directory and run:
   ```bash
   cargo build --release
   ```

3. **Run the Application**:
   To start the web application, you can use a simple HTTP server. If you have Python installed, you can run:
   ```bash
   python -m http.server 8000
   ```
   or use any other server of your choice.

4. **Open in Your Browser**:
   Visit `http://localhost:8000` in your browser to access the application.

5. **Using the App**:
   - Drag and drop multiple images onto the designated area or click to upload files.
   - Once you have uploaded at least two images, the "Process HDR" button will become enabled.
   - Click the "Process HDR" button to create your HDR image.
   - The final HDR image will be displayed on the screen for you to save or share!

## Tech Info ⚙️

- **Frontend Technologies**: HTML, CSS, and JavaScript
- **Backend Processing**: 
  - **Rust** with **Wasm** for efficient image processing.
  - Uses various crates such as `image`, `exif`, and `serde` for handling images and data serialization.
- **Key Features**:
  - Supports images taken with exposure bracketing (e.g., -2EV, 0EV, +2EV).
  - Extracts and displays exposure metadata from images.
  - Utilizes modern web capabilities to provide a smooth user experience.

### Project Structure 📂
```
.
├── Cargo.lock
├── Cargo.toml
├── index.html
├── static/
│   ├── css/
│   │   └── style.css
│   ├── js/
│   │   └── main.js
│   └── index.html
├── .gitignore
└── src/
    └── lib.rs
```

### Dependencies 📦
- `wasm-bindgen`: For generating bindings between Rust and JavaScript.
- `image`: For image processing operations.
- `serde`: For serializing and deserializing data.
- `console_error_panic_hook`: For better error handling in Wasm applications.

For more details on the individual packages, check the `Cargo.toml` file in the project.

---

If you have any questions or issues, feel free to open an issue on this repository! Happy merging! 🖼️✨
