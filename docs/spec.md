Here’s a concise spec document for your HDR web app:

---

## **HDR Image Creator: Web App Specification**

### **Overview**
A simple, responsive web application to create HDR images from 3-5 uploaded photos using a Rust-based HDR processing library. The app supports JPG and DNG files, validates exposures, and provides both auto-processing and advanced tuning options.

---

### **Key Features**
1. **Image Upload**:
   - Supports JPG and DNG formats.
   - Drag-and-drop and file picker options.
   - Validates images for:
     - File type and EXIF data presence.
     - Exposure sequence (-2, 0, +2 or similar pattern).
   - Displays detailed error messages for invalid images.

2. **HDR Processing**:
   - Automatically orders images by exposure based on EXIF data.
   - Uses the Rust `image-hdr` crate for HDR generation.
   - Displays a progress bar with clear step breakdown (validation, HDR generation, rendering preview).

3. **User Interaction**:
   - Preview with compare-and-contrast slider (HDR vs. original middle exposure).
   - Advanced tuning via sliders for brightness, contrast, and other parameters.
   - Light/dark mode toggle.
   - Option to reset all inputs and preferences.

4. **Output**:
   - Generates HDR image in DNG or JPG format.
   - Downloads file with a filename based on the middle image (e.g., `IMG_1237_HDR.DNG`).
   - Displays estimated file size before download.

5. **Feedback & Notifications**:
   - Confetti animation on successful HDR generation.
   - Clear error messages and retry options for failed processes.
   - Warning before navigating away if HDR image is not downloaded.

6. **Preferences**:
   - Saves tuning preferences in local storage for reuse.
   - Resets preferences on user request.

---

### **Architecture**
1. **Frontend**:
   - Framework: Next.js (React-based SPA).
   - Styling: Tailwind CSS.
   - Features:
     - Responsive, mobile-friendly UI.
     - Drag-and-drop file upload.
     - Compare-and-contrast slider.
     - Progress bar with detailed steps.
     - Light/dark mode toggle.
   - Accessibility:
     - Full keyboard navigation and ARIA support.
     - Screen reader compatibility.
     - High-contrast mode.

2. **Backend (via WASM)**:
   - **Rust HDR Library**:
     - Uses `image-hdr` crate for processing.
     - Parses EXIF data to validate images and determine exposure order.
   - **WASM Integration**:
     - Rust library compiled to WebAssembly for client-side execution.
   - Error Handling:
     - Detailed feedback for invalid files or processing errors.

3. **Storage**:
   - Local storage for:
     - Tuning preferences.
     - Offline app usage (via service worker).

---

### **App Flow**
1. **Image Upload**:
   - User uploads 3-5 images.
   - App validates file type and exposure sequence.
   - Displays EXIF metadata for images.

2. **Processing**:
   - HDR generation starts upon successful validation.
   - Progress bar updates with:
     - Validation complete.
     - HDR generation in progress.
     - Preview rendering.

3. **Preview & Adjustment**:
   - Displays HDR preview with a compare-and-contrast slider.
   - Advanced mode enables slider adjustments (brightness, contrast, etc.).

4. **Download**:
   - User downloads HDR image in DNG or JPG.
   - Displays confirmation message with confetti animation.

5. **Reset**:
   - Clears images, resets preferences, and returns to upload state.

---

### **Deployment**
- **Platform**: Netlify (for static hosting with WASM support).
- **Offline Capability**: Service worker for caching app assets.
- **Languages Supported**: English and Japanese (localization-ready).

---

### **Additional Notes**
- Batch processing is not supported (one HDR image at a time).
- No analytics or telemetry to maintain privacy.
- UI inspired by Kai’s Power Tools, with subtle animations for enhanced UX.

---

Let me know if this needs tweaking or expansion!
