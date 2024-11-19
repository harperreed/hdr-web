# TODO List for HDR Image Creator

## 🚀 **Project Setup**
- [ ] Set up a Next.js project for the frontend.
- [ ] Configure Tailwind CSS for styling.
- [ ] Compile the Rust `image-hdr` library to WebAssembly (WASM).
- [ ] Create a basic Netlify deployment for testing.

---

## 🖼️ **Image Upload**
- [ ] Implement drag-and-drop and file picker for image uploads.
- [ ] Validate uploaded images:
  - [ ] Check file format (JPG/DNG).
  - [ ] Parse and display EXIF data.
  - [ ] Verify exposure sequence.
  - [ ] Show detailed error messages for invalid files.
- [ ] Automatically order images by exposure.

---

## 🌟 **HDR Processing**
- [ ] Integrate Rust library via WASM for HDR generation.
- [ ] Display a progress bar with steps:
  - [ ] Validation complete.
  - [ ] HDR generation in progress.
  - [ ] Rendering preview.
- [ ] Add error handling for failed HDR generation.
- [ ] Allow users to retry processing without re-uploading files.

---

## 🖌️ **Preview & Tuning**
- [ ] Implement HDR preview with compare-and-contrast slider.
- [ ] Add advanced tuning sliders:
  - [ ] Brightness.
  - [ ] Contrast.
  - [ ] Other parameters (TBD).
- [ ] Reset tuning sliders to defaults.

---

## 📂 **Download**
- [ ] Generate HDR image in DNG and JPG formats.
- [ ] Derive filename from the middle image (e.g., `IMG_1237_HDR.DNG`).
- [ ] Display estimated file size before download.
- [ ] Add confetti animation on successful download.

---

## 🛠️ **User Interface**
- [ ] Design a responsive layout (mobile and desktop).
- [ ] Add light/dark mode toggle.
- [ ] Create reset button to clear uploaded images and preferences.
- [ ] Warn users before navigating away without downloading.

---

## 🌐 **Localization**
- [ ] Add English and Japanese language support.
- [ ] Store language preferences in LocalStorage.

---

## ♿ **Accessibility**
- [ ] Ensure keyboard navigation support.
- [ ] Add ARIA attributes for screen readers.
- [ ] Implement high-contrast mode.

---

## 🔧 **Testing**
- [ ] Write unit tests for the Rust library:
  - [ ] EXIF data parsing.
  - [ ] Exposure validation.
  - [ ] HDR generation.
- [ ] Write integration tests for WASM and frontend:
  - [ ] File validation and ordering.
  - [ ] Slider adjustments.
  - [ ] HDR generation and download.
- [ ] Perform manual testing for:
  - [ ] Responsiveness (mobile and desktop).
  - [ ] Accessibility compliance (WCAG standards).
- [ ] Test offline functionality with service workers.

---

## 📦 **Deployment**
- [ ] Configure Netlify for deployment.
- [ ] Add service worker for offline support.
- [ ] Optimize performance (e.g., WASM load times).

---

## ✨ **Enhancements (Future)**
- [ ] Add batch processing support.
- [ ] Include more advanced tuning parameters.
- [ ] Create a help section or tooltips for user guidance.
- [ ] Add support for more languages (beyond English and Japanese).
