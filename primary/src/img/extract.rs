use image::{Rgb, imageops::FilterType};

pub struct ImageFeature {
    pub pixels_data: Vec<Vec<(u8, u8, u8)>>,
}

impl ImageFeature {
    pub fn extract_pixels_feature() -> Self {
        let img = image::ImageReader::open("image/sample.jpg")
            .expect("Failed to open image")
            .decode()
            .expect("Failed to decode image")
            .to_rgb8();

        // image resized to 400x300
        let img = image::imageops::resize(&img, 400, 300, FilterType::Lanczos3);
        
        println!("Image resized to 400x300 pixels");

        let (width, height) = img.dimensions();

        // 2D vector to store rgb values
        let mut pixels_data_temp: Vec<Vec<(u8, u8, u8)>> = Vec::with_capacity(height as usize);

        for y in 0..height {
            let mut row: Vec<(u8, u8, u8)> = Vec::with_capacity(width as usize);
            for x in 0..width {
                let pixel: Rgb<u8> = *img.get_pixel(x, y);
                row.push((pixel[0], pixel[1], pixel[2]));
            }
            pixels_data_temp.push(row);
        }

        ImageFeature {
            pixels_data: pixels_data_temp,
        }
    }
}
