use crate::charify::Charify;
use image::imageops::{colorops::BiLevel, dither};
use image::{DynamicImage, GenericImage, GenericImageView, ImageBuffer, Luma, Primitive};
use std::ops::Deref;

use super::utils::get_view;

pub struct BrailleCharMap;

// TODO reimplement dithering with more polymorphyc function
impl<'a> Charify<'a, Luma<u8>, u8, Vec<u8>> for BrailleCharMap {
    fn charify(&self, image: &ImageBuffer<Luma<u8>, Vec<u8>>) -> String {
        let scale = [
            '⠀', '⠠', '⠐', '⠰', '⠈', '⠨', '⠘', '⠸', '⠄', '⠤', '⠔', '⠴', '⠌', '⠬', '⠜', '⠼', '⠂',
            '⠢', '⠒', '⠲', '⠊', '⠪', '⠚', '⠺', '⠆', '⠦', '⠖', '⠶', '⠎', '⠮', '⠞', '⠾', '⠁', '⠡',
            '⠑', '⠱', '⠉', '⠩', '⠙', '⠹', '⠅', '⠥', '⠕', '⠵', '⠍', '⠭', '⠝', '⠽', '⠃', '⠣', '⠓',
            '⠳', '⠋', '⠫', '⠛', '⠻', '⠇', '⠧', '⠗', '⠷', '⠏', '⠯', '⠟', '⠿',
        ];
        let width: u32 = 2;
        let height: u32 = 3;
        let image = &mut image.clone();
        dither(image, &BiLevel);
        let mut res = String::with_capacity(((image.width() + 1) * image.height()) as usize);
        let max_index = (scale.len() - 1) as f32;

        for y in (0..image.height() - 1).step_by(height as usize + 1) { // + 1 cos there is small gap between lines
            for x in (0..image.width()).step_by(width as usize) {
                if x == 0 && y != 0 {
                    res.push('\n');
                }
                if let Some(view) = get_view(image, x, y, width, height) {
                    let view_size = (view.height() * view.width()) as f32;
                    let brightness: f32 = view
                        .pixels()
                        .map(|(.., pix)| pix.0[0] as f32 / 255.0)
                        .sum::<f32>()
                        / view_size;
                    let index = (brightness * max_index) as usize;
                    let b = *scale.get(index).unwrap();
                    res.push(b);
                }
            }
        }
        res
    }
}
