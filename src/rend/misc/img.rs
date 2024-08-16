// use std::path::Path;

use std::path::Path;

use fast_image_resize::{images::Image, IntoImageView, ResizeAlg, ResizeOptions, Resizer};
use image::{save_buffer, DynamicImage, ExtendedColorType, ImageResult};
use uuid::Uuid;


pub fn save(path: &Path, image: &Image) -> ImageResult<()> {
    let id:Uuid = Uuid::new_v4();
    let image_path = path.join(&format!("{}-w_{}_h_{}.png",id,image.width(),image.height()));
    return save_buffer(&image_path, image.buffer(), image.width(),image.height(), ExtendedColorType::Rgba8);
}


pub fn resize(image: &DynamicImage, width: u32, height: u32) -> Image {
    let mut dst_image = Image::new(
        width,height,
        image.pixel_type().unwrap(),
        
    );
    let mut resizer = Resizer::new();
    resizer.resize(image, &mut dst_image, &ResizeOptions::new().resize_alg(ResizeAlg::Nearest)).unwrap();
    return dst_image;
}
