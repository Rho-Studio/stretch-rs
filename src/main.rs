use crate::extensions::NDArrayBuffer;
use image::{DynamicImage, GenericImageView, ImageBuffer, Rgb};
use stretchrs;
use stretchrs::gamma::Stretcher;

mod extensions;

fn main() {
    let image = image::open("Untitled.tif").unwrap();
    let (width, height) = image.dimensions();

    let mut buffer = image.to_nd_array_buffer();

    let stretch = Stretcher::from_image(&image);

    stretch.apply(&mut buffer);

    let mut image = ImageBuffer::<Rgb<u16>, Vec<u16>>::new(width, height);

    for (x, y, pixel) in image.enumerate_pixels_mut() {
        *pixel = Rgb([
            (buffer[[y as usize, x as usize, 0]] * u16::MAX as f32) as u16,
            (buffer[[y as usize, x as usize, 1]] * u16::MAX as f32) as u16,
            (buffer[[y as usize, x as usize, 2]] * u16::MAX as f32) as u16,
        ]);
    }

    image.save("out.tiff").unwrap();
}
