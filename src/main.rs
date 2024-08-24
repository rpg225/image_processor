use image::{self, RgbImage};
use image::imageops;

fn blur(image: &RgbImage, kernel_size: f32) -> RgbImage {
    let blurred = imageops::blur(image, kernel_size);
    blurred
}

fn main() {
    let img = image::open("C:\\Users\\Rambod\\image_processor\\src\\assets\\blur.jpg").unwrap().to_rgb8();
    let blurred = blur(&img, 3.0);
    blurred.save("C:\\Users\\Rambod\\image_processor\\src\\assets\\blurred.jpg").unwrap();
}