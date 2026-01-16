use super::extract::ImageFeature;

pub struct ComputeConvolution {
    pub computed_pixels: Vec<Vec<(u8, u8, u8)>>,
}

impl ComputeConvolution {
    pub fn extract_pixels_feature(pixels_data: &ImageFeature) -> Self {
        let kernel: [[i32; 3]; 3] = [[0, -1, 0], [-1, 5, -1], [0, -1, 0]];

        let height = pixels_data.pixels_data.len();
        let width = pixels_data.pixels_data[0].len();

        let mut computed_pixels = vec![vec![(0u8, 0u8, 0u8); width]; height];

        // Loop over each pixel
        for y in 1..(height - 1) {
            for x in 1..(width - 1) {
                let mut sum_r: i32 = 0;
                let mut sum_g: i32 = 0;
                let mut sum_b: i32 = 0;

                // Apply kernel
                for ky in 0..3 {
                    for kx in 0..3 {
                        let pixel = pixels_data.pixels_data[y + ky - 1][x + kx - 1];
                        let weight = kernel[ky][kx];
                        sum_r += pixel.0 as i32 * weight;
                        sum_g += pixel.1 as i32 * weight;
                        sum_b += pixel.2 as i32 * weight;
                    }
                }

                // clamping values to 0..255
                let r = sum_r.clamp(0, 255) as u8;
                let g = sum_g.clamp(0, 255) as u8;
                let b = sum_b.clamp(0, 255) as u8;

                computed_pixels[y][x] = (r, g, b);
            }
        }

        println!(
            "Image size: {}x{}",
            computed_pixels[0].len(),
            computed_pixels.len()
        );

        // println!(
        //     "RBG value of a particular pixel: {:?}",
        //     computed_pixels[200][150]
        // );

        ComputeConvolution { computed_pixels }
    }
}
