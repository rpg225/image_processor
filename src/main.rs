use image::{open, DynamicImage, imageops::FilterType};

fn resize_image(path: &str, width: u32, height: u32) -> DynamicImage {
 let img = open(path).expect("Failed to open image");
 img.resize(width, height, FilterType::Lanczos3)
}

fn save_image(img: &DynamicImage, output_path: &str) {
    img.save_with_format(output_path, image::ImageFormat::Png).expect("Failed to save image");
}

fn main() {
    println! ("Image Processing");
    let resized_image =  resize_image(
        "C:\\Users\\Rambod\\image_processor\\src\\assets\\sample_img.jpg",
        512,
        512

    );

    save_image(
        &resized_image,
        "C:\\Users\\Rambod\\image_processor\\src\\assets\\sample_img_Resized.png"
    );
}
