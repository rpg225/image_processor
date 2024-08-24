use image::{DynamicImage, ImageBuffer, Rgba};
use rayon::prelude::*;
use std::fs;
use std::path::Path;

fn load_image(filepath: &str) -> Result<DynamicImage, image::ImageError> {
    image::open(filepath)
}

fn rotate_image_90_clockwise(img: &ImageBuffer<Rgba<u8>, Vec<u8>>,) -> ImageBuffer<Rgba<u8>, Vec<u8>> {
    let (width, height) = img.dimensions();

    let mut new_img = ImageBuffer::new(height, width); // new image dimensions are swapped

    img.enumerate_pixels().for_each( | (x, y, pixel ) | {
        let new_x = height - y - 1;
        let new_y = x;
        new_img.put_pixel(new_x, new_y, *pixel);

    });

    new_img

}


fn main() {
    println! ("Image Processing - Parallel Processing through Rayon");

    // get all image paths in the parallel directory
    let image_dir = Path::new("parallel");
    let image_paths = fs::read_dir(image_dir)
        .expect("Failed to read directory")
        .into_tier()
        .filter_map(Result::ok)
        .filter(|entry| entry.file_type().is_ok() && entry.file_type().unwrap().is_file())
        .map(|entry| entry.path())
        .collect::<Vec<_>>();

        image_paths.par_iter().for_each(|img_path| {
            let img = load_image(img_path.to_str().unwrap()).expect("Failed to load image");
            let rotated_img = rotate_image_90_clockwise(&img.to_rgba8()); // rotate 90 degrees clockwise

            let processed_path = format!("parallel/processed_{}", img_path.file_name().unwrap().to_str().unwrap());
            rotated_img
                .save(processed_path)
                .expect("Failed to save processed file.");

        });

    // let rotated_img = rotate_image_90_clockwise(&img.to_rgba8()); // rotate 90 degrees clockwise

    // rotated_img
    //     .save("C:\\Users\\Rambod\\image_processor\\src\\assets\\processedimg.png")
    //     .expect("Failed to save processed image.");


    // cropped_img.save("C:\\Users\\Rambod\\image_processor\\src\\assets\\croppedimg.png").expect("Failed to save cropped image");

    // let resized_image = resize_image_maintain_ratio(
    // "C:\\Users\\Rambod\\image_processor\\src\\assets\\sample_img.jpg",
    // Some(800),
    // None
    // );

    // save_image(
    //     &cropped_img,
    //     "C:\\Users\\Rambod\\image_processor\\src\\assets\\croppedimg.png"
    // );

}
