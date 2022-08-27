use ::std::ops::Deref;
use image::{GenericImageView, ImageBuffer, Pixel, SubImage};

/// gets croped valid subimage or `None` also read [`image::GenericImageView::view`]
pub fn get_view_checked<P, T, C>(
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
    let mut height = height;
    let mut width = width;
    if x + width > image.width() {
        width = image.width().saturating_sub(x);
    } 
    if y + height > image.height() {
        height = image.height().saturating_sub(y);
    }

    if width == 0 || height == 0 {
        None
    } else {
        Some(image.view(x, y, width, height))
    }
}
