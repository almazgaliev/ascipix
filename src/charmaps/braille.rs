use crate::charify::Charify;
use image::imageops::{colorops::BiLevel, dither};
use image::{GenericImageView, ImageBuffer, Luma};

use super::utils::get_view;

pub struct BrailleCharMap;

// TODO reimplement dithering with more polymorphyc function
impl<'a> Charify<'a, Luma<u8>, u8, Vec<u8>> for BrailleCharMap {
    fn charify(&self, image: &ImageBuffer<Luma<u8>, Vec<u8>>) -> String {
        let width: u32 = 2;
        let height: u32 = 4;
        let image = &mut image.clone();
        dither(image, &BiLevel);
        let mut res = String::with_capacity(((image.width() + 1) * image.height()) as usize);

        for y in (0..image.height() - 1).step_by(height as usize) {
            // + 1 cos there is small gap between lines
            for x in (0..image.width()).step_by(width as usize) {
                if x == 0 && y != 0 {
                    res.push('\n');
                }
                let mut ch = '_';
                if let Some(view) = get_view(image, x, y, width, height) {
                    let points = view
                        .pixels()
                        .map(|(x, y, pix)| (x, y,((pix.0[0] as f32 / 255.0).round() as u32)));
                    ch = to_braille(points);
                }
                res.push(ch);
            }
        }
        res
    }
}

#[inline(always)]
fn to_braille(points: impl Iterator<Item = (u32, u32, u32)>) -> char {
    let mut ch: u32 = 0;
    for (x, y, point) in points {
        ch = ch | (point << order((y, x)));
    }
    unsafe { char::from_u32_unchecked(0x2800 + ch) }
}

#[inline(always)]
fn order(coords: (u32, u32)) -> u32 {
    match coords {
        (0, 0) => 0,
        (0, 1) => 1,
        (1, 0) => 2,
        (1, 1) => 3,
        (2, 0) => 4,
        (2, 1) => 5,
        (3, 0) => 6,
        (3, 1) => 7,
        _ => panic!("braille with 8 dots only uses 8 bits, if you see this then subimage view is too big"),
    }
    // let order: [u32; 8] = [0, 2, 4, 1, 3, 5, 6, 7];
}
