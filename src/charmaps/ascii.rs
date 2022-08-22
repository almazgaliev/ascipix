use crate::charify::Charify;
use image::{GenericImageView, ImageBuffer, Luma, Primitive};
use std::ops::Deref;

use super::utils::get_view;
pub struct GrayCharMap(Vec<char>);

impl Default for GrayCharMap {
    fn default() -> Self {
        Self("WHyzv;\"".chars().rev().collect())
    }
}

impl GrayCharMap {
    pub fn new(grayscale: Vec<char>) -> Self {
        Self(grayscale)
    }
    pub fn existing(index: usize) -> Self {
        let grayscales = ["WHyzv;\"` ", "NWyx?!`. ", "@%#*+=-:. ", "@$#*!=;:~-,. "];
        match grayscales.get(index) {
            Some(slice) => Self(slice.chars().rev().collect()),
            None => panic!("index: {} is out of bounds", index),
        }
    }
}

impl<'a, C, T> Charify<'a, Luma<T>, T, C> for GrayCharMap
where
    T: Primitive,
    C: Deref<Target = [T]>,
{
    fn charify(&self, image: &ImageBuffer<Luma<T>, C>) -> String {
        let width: u32 = 1;
        let height: u32 = 2;

        let mut res = String::with_capacity(((image.width() + 1) * image.height()) as usize);
        let max_index = (self.0.len() - 1) as f32;

        for y in (0..image.height() - 1).step_by(height as usize) {
            for x in (0..image.width()).step_by(width as usize) {
                if x == 0 && y != 0 {
                    res.push('\n');
                }
                if let Some(view) = get_view(image, x, y, width, height) {

                    let view_size = (view.height() * view.width()) as f32;
                    let brightness: f32 = view
                        .pixels()
                        .map(|(.., pix)| pix.0[0].to_f32().unwrap() / 255.0)
                        .sum::<f32>()
                        / view_size;
                    let index = (brightness * max_index) as usize;
                    let b = *self.0.get(index).unwrap();
                    res.push(b);
                }
            }
        }
        res
    }
}
