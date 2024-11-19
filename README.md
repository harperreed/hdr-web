# HDR Web App

A web application for creating HDR images from multiple exposures using Next.js and Rust WASM.

## Setup

1. Install dependencies:
   ```bash
   cd frontend && npm install
   cd ../rust-hdr && cargo build
   ```

2. Run development server:
   ```bash
   cd frontend && npm run dev
   ```

## Development

- Frontend: http://localhost:3000
- Build WASM: `cd rust-hdr && wasm-pack build`
