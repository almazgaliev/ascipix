use ::std::ops::Deref;
use image::{GenericImageView, ImageBuffer, Pixel, SubImage};

pub fn get_view<P, T, C>(
    image: &ImageBuffer<P, C>,
    x: u32,
    y: u32,
    width: u32,
    height: u32,
) -> Option<SubImage<&ImageBuffer<P, C>>>
where
    P: Pixel<Subpixel = T>,
    C: Deref<Target = [T]>,
{
    if x + width > image.width() {
        None
    } else if y + height > image.height() {
        None
    } else if width == 0 || height == 0 {
        None
    } else {
        Some(image.view(x, y, width, height))
    }
}
