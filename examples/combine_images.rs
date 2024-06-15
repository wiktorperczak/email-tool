extern crate image;

use image::{GenericImageView, RgbaImage, Rgba};

fn main() {
    // Load the images
    let img1 = image::open("plots/histogram.png").expect("Failed to open image 1").to_rgba8();
    let img2 = image::open("plots/histogram.png").expect("Failed to open image 2").to_rgba8();
    let img3 = image::open("plots/histogram.png").expect("Failed to open image 3").to_rgba8();
    let img4 = image::open("plots/histogram.png").expect("Failed to open image 4").to_rgba8();

    // Get the dimensions of the images
    let (width1, height1) = img1.dimensions();
    let (width2, height2) = img2.dimensions();
    let (width3, height3) = img3.dimensions();
    let (width4, height4) = img4.dimensions();

    // Ensure all images have the same dimensions (for simplicity)
    assert!(width1 == width2 && width1 == width3 && width1 == width4);
    assert!(height1 == height2 && height1 == height3 && height1 == height4);

    // Create a new image with the combined dimensions
    let mut combined_image = RgbaImage::new(width1 * 2, height1 * 2);

    // Copy images into the combined image
    for (x, y, pixel) in img1.enumerate_pixels() {
        combined_image.put_pixel(x, y, *pixel);
    }

    for (x, y, pixel) in img2.enumerate_pixels() {
        combined_image.put_pixel(x + width1, y, *pixel);
    }

    for (x, y, pixel) in img3.enumerate_pixels() {
        combined_image.put_pixel(x, y + height1, *pixel);
    }

    for (x, y, pixel) in img4.enumerate_pixels() {
        combined_image.put_pixel(x + width1, y + height1, *pixel);
    }

    // Save the combined image
    combined_image.save("plots/combined_image.png").expect("Failed to save combined image");
}
