use image::{open, DynamicImage, imageops::FilterType, ImageFormat};
use image::GenericImageView; // required  for the the dimensions

fn save_image(img: &DynamicImage, output_path: &str) {
    img.save_with_format(output_path, image::ImageFormat::Png).expect("Failed to save image");
}

fn resize_image_maintain_ratio(path: &str, new_width: Option<u32>, new_height: Option<u32>) -> DynamicImage {
    let img = open(path).expect("Failed to open image");
    let (width, height) = img.dimensions();

    let ratio = width as f32 / height as f32;
    let (resize_width, resize_height) = match(new_width, new_height){
        (Some(w), None) => (w,(w as f32 / ratio).round() as u32),
        (None, Some(h)) => ((h as f32 * ratio).round() as u32, h),
        (Some(w), Some(h)) => (w, h), // if both dimension are specified as is
        (None, None) => (width, height), // if no dimensions are specified use the original dimension
    };

    img.resize(resize_width, resize_height, FilterType::Lanczos3)
}



fn main() {
    println! ("Image Processing");

    let resized_image = resize_image_maintain_ratio(
    "C:\\Users\\Rambod\\image_processor\\src\\assets\\sample_img.jpg",
    Some(800),
    None
    );

    save_image(
        &resized_image,
        "C:\\Users\\Rambod\\image_processor\\src\\assets\\sample_img_RE_DI.png"
    );

}
