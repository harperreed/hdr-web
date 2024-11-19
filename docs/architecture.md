Here’s a simple architecture for the app:

---

### **Frontend**
- **Framework**: Next.js (for ease of building a single-page app with responsive UI)
- **Styling**: Tailwind CSS (for customizable and quick styling, supporting light/dark mode easily)
- **Features**:
  - File upload UI with drag-and-drop support.
  - Progress bar with step tracking.
  - Compare-and-contrast slider for HDR previews.
  - Light/dark mode toggle.
  - Reset and download buttons.
  - LocalStorage for saving tuning preferences.

---

### **Backend**
- **Rust Library Integration**: 
  - Use **WASM** (compiled from the Rust library) for running the `image-hdr` crate directly in the browser.
  - Validates images (checks file type, exposure sequence).
  - Processes HDR generation using the library.
  - Extracts EXIF data and orders images automatically.

---

### **App Flow**
1. **Upload Images**:
   - Users upload 3-5 images in JPG/DNG format.
   - Frontend sends files to the Rust library via WASM for validation.
   - If validation passes, EXIF data is displayed, and images are ordered.

2. **Process HDR**:
   - Once validated, HDR generation is triggered through the Rust library.
   - A progress bar updates as steps are completed (validation, generation, rendering preview).

3. **Preview & Adjust**:
   - Auto-generated HDR is displayed with a compare-and-contrast slider.
   - Advanced tuning options (via sliders) allow further adjustments.

4. **Download**:
   - User downloads the HDR image in DNG/JPG format.
   - Filename derived from the middle file.

5. **Reset**:
   - Clears all images and preferences, ready for new input.

---

### **Key Tools & Libraries**
- **Frontend**:
  - Next.js
  - Tailwind CSS
  - A11y libraries (ARIA attributes, keyboard support)
- **Backend/WASM**:
  - Rust `image-hdr` crate
  - Rust EXIF parser for metadata handling
- **Additional Features**:
  - Confetti animations: Use a lightweight library like `canvas-confetti`.
  - Compare-and-contrast slider: Simple custom implementation with HTML and JavaScript.

---

### **Deployment**
- **Hosting**: Netlify (static hosting with WASM support)
- **Offline Support**: Service worker for caching app assets.

---

Thoughts? Adjustments?
