use wasm_bindgen::prelude::*;
use image::{DynamicImage, GenericImageView, ImageBuffer, Rgb};
use std::io::Cursor;
use base64::{Engine as _, engine::general_purpose::STANDARD as BASE64};
use exif::{Reader, In, Tag, Value};
use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Clone)]
pub struct ImageInfo {
    exposure_time: String,
    iso: Option<u32>,
    exposure_bias: Option<f32>,
    index: usize,
}

#[wasm_bindgen]
pub struct HdrProcessor {
    images: Vec<(DynamicImage, ImageInfo)>,
}

fn get_rational_as_f32(value: &Value) -> Option<f32> {
    if let Value::Rational(ref vec) = value {
        if !vec.is_empty() {
            let rational = vec[0];
            return Some(rational.num as f32 / rational.denom as f32);
        }
    }
    None
}

fn extract_exif_data(data: &[u8], index: usize) -> Result<ImageInfo, String> {
    let mut cursor = Cursor::new(data);
    let exif = Reader::new()
        .read_from_container(&mut cursor)
        .map_err(|e| format!("EXIF error: {}", e))?;

    let exposure_time = exif
        .get_field(Tag::ExposureTime, In::PRIMARY)
        .map(|field| field.display_value().to_string())
        .unwrap_or_else(|| "Unknown".to_string());

    let iso = exif
        .get_field(Tag::PhotographicSensitivity, In::PRIMARY)
        .and_then(|field| match field.value {
            Value::Short(ref vec) if !vec.is_empty() => Some(vec[0] as u32),
            Value::Long(ref vec) if !vec.is_empty() => Some(vec[0] as u32),
            _ => None
        });

    let exposure_bias = exif
        .get_field(Tag::ExposureBiasValue, In::PRIMARY)
        .and_then(|field| get_rational_as_f32(&field.value));

    Ok(ImageInfo {
        exposure_time,
        iso,
        exposure_bias,
        index,
    })
}

fn sort_by_exposure(images: &mut [(DynamicImage, ImageInfo)]) {
    images.sort_by(|a, b| {
        // First try to sort by exposure bias
        if let (Some(bias_a), Some(bias_b)) = (a.1.exposure_bias, b.1.exposure_bias) {
            return bias_a.partial_cmp(&bias_b).unwrap_or(std::cmp::Ordering::Equal);
        }
        
        // If no exposure bias, try to sort by exposure time
        let parse_exposure = |exp: &str| -> f32 {
            exp.split('/')
                .map(|s| s.parse::<f32>().unwrap_or(1.0))
                .reduce(|a, b| a / b)
                .unwrap_or(0.0)
        };
        
        let time_a = parse_exposure(&a.1.exposure_time);
        let time_b = parse_exposure(&b.1.exposure_time);
        time_a.partial_cmp(&time_b).unwrap_or(std::cmp::Ordering::Equal)
    });
}

fn merge_exposures(images: &[(DynamicImage, ImageInfo)]) -> DynamicImage {
    if images.is_empty() {
        panic!("No images provided");
    }

    let width = images[0].0.width();
    let height = images[0].0.height();
    let size = (width * height) as usize;
    
    let mut accumulator = vec![0.0f32; size * 3];
    let mut weights = vec![0.0f32; size];
    
    // Weight calculation based on exposure
    for (img, info) in images {
        let exposure_value = if let Some(bias) = info.exposure_bias {
            2.0f32.powf(bias)
        } else {
            1.0
        };

        for (x, y, pixel) in img.pixels() {
            let idx = (y * width + x) as usize;
            let rgb_idx = idx * 3;
            
            // Calculate weight based on pixel brightness and exposure
            let brightness = (pixel[0] as f32 + pixel[1] as f32 + pixel[2] as f32) / (3.0 * 255.0);
            let weight = if brightness < 0.1 || brightness > 0.9 {
                0.0
            } else {
                // Give more weight to properly exposed pixels
                let dist_from_mid = (brightness - 0.5).abs();
                1.0 - (dist_from_mid * 2.0)
            };
            
            accumulator[rgb_idx] += pixel[0] as f32 * weight * exposure_value;
            accumulator[rgb_idx + 1] += pixel[1] as f32 * weight * exposure_value;
            accumulator[rgb_idx + 2] += pixel[2] as f32 * weight * exposure_value;
            weights[idx] += weight;
        }
    }

    // Create output buffer with tone mapping
    let mut merged = ImageBuffer::new(width, height);
    
    for y in 0..height {
        for x in 0..width {
            let idx = (y * width + x) as usize;
            let rgb_idx = idx * 3;
            
            let weight = weights[idx].max(1.0); // Avoid division by zero
            
            let r = accumulator[rgb_idx] / (weight * 255.0);
            let g = accumulator[rgb_idx + 1] / (weight * 255.0);
            let b = accumulator[rgb_idx + 2] / (weight * 255.0);
            
            // Improved tone mapping with gamma correction
            let gamma = 2.2;
            let mapped_r = ((r / (1.0 + r)).powf(1.0 / gamma) * 255.0).clamp(0.0, 255.0) as u8;
            let mapped_g = ((g / (1.0 + g)).powf(1.0 / gamma) * 255.0).clamp(0.0, 255.0) as u8;
            let mapped_b = ((b / (1.0 + b)).powf(1.0 / gamma) * 255.0).clamp(0.0, 255.0) as u8;
            
            merged.put_pixel(x, y, Rgb([mapped_r, mapped_g, mapped_b]));
        }
    }

    DynamicImage::ImageRgb8(merged)
}

#[wasm_bindgen]
impl HdrProcessor {
    #[wasm_bindgen(constructor)]
    pub fn new() -> Self {
        console_error_panic_hook::set_once();
        HdrProcessor {
            images: Vec::new(),
        }
    }

    #[wasm_bindgen]
    pub fn add_image(&mut self, base64_data: &str) -> Result<JsValue, JsValue> {
        let data = BASE64.decode(base64_data.split(',').last().unwrap_or(""))
            .map_err(|e| JsValue::from_str(&e.to_string()))?;
        
        let img = image::load_from_memory(&data)
            .map_err(|e| JsValue::from_str(&e.to_string()))?;
        
        let index = self.images.len();
        let info = extract_exif_data(&data, index)
            .map_err(|e| JsValue::from_str(&e.to_string()))?;
        
        self.images.push((img, info.clone()));
        
        // Return the image info to display in UI
        serde_wasm_bindgen::to_value(&info).map_err(|e| JsValue::from_str(&e.to_string()))
    }

    #[wasm_bindgen]
    pub fn process_hdr(&mut self) -> Result<String, JsValue> {
        if self.images.len() < 2 {
            return Err(JsValue::from_str("Need at least 2 images for HDR"));
        }

        // Sort images by exposure
        sort_by_exposure(&mut self.images);
        
        // Process HDR using our in-memory implementation
        let merged = merge_exposures(&self.images);
        
        // Convert result to base64
        let mut buffer = Cursor::new(Vec::new());
        merged.write_to(&mut buffer, image::ImageFormat::Png)
            .map_err(|e| JsValue::from_str(&e.to_string()))?;
        
        let base64 = format!(
            "data:image/png;base64,{}",
            BASE64.encode(&buffer.into_inner())
        );
        
        Ok(base64)
    }
}
