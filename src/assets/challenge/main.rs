use image::{self, RgbImage};
use image::imageops;

// Apply Gaussian blur to an image and save the result
fn blur(image: &RgbImage, kernel_size: f32) -> RgbImage {
    // Apply Gaussian blur with specified kernel size
    let blurred = imageops::blur(image, kernel_size);

    blurred
}

fn main() {
    // Load an image from a file
    let img = image::open("assets/blur.jpg").unwrap().to_rgb8();

    // Apply Gaussian blur with kernel size 3.0
    let blurred = blur(&img, 3.0);

    // Save the result
    blurred.save("assets/blurred.jpg").unwrap();
}
