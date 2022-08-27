
use crate::charify::Charify;
use image::{GenericImageView, ImageBuffer, Luma, Primitive};
use std::ops::Deref;

use super::utils::get_view_checked;
pub struct GrayCharMap {
    grayscale: Vec<char>,
}

impl Default for GrayCharMap {
    fn default() -> Self {
        Self {
            grayscale: " `\";vzyHW".chars().collect(),
        }
    }
}

impl GrayCharMap {
    pub fn new(grayscale: String) -> Self {
        Self {
            grayscale: grayscale.chars().collect(),
        }
    }
    /// returns nth CharMap from this list:
    /// * " `\";vzyHW",
    /// * " .`!?xyWN",
    /// * " .:-=+*#%@",
    /// * " .,-~:;=!*#$@",
    /// * " .'`^\",:;Il!i><~+_-?][}{1)(|\\/tfjrxnuvczXYUJCLQ0OZmwqpdbkhao*#MW&8%B;@$",
    pub fn from_existing(index: usize) -> Self {
        // Reference http://paulbourke.net/dataformats/asciiart/
        let grayscales = [
            " `\";vzyHW",
            " .`!?xyWN",
            " .:-=+*#%@",
            " .,-~:;=!*#$@",
            " .'`^\",:;Il!i><~+_-?][}{1)(|\\/tfjrxnuvczXYUJCLQ0OZmwqpdbkhao*#MW&8%B;@$",
        ];
        match grayscales.get(index) {
            Some(slice) => Self {
                grayscale: slice.chars().collect(),
            },
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
        let max_index = (self.grayscale.len() - 1) as f32;

        for y in (0..image.height() - 1).step_by(height as usize) {
            for x in (0..image.width()).step_by(width as usize) {
                if x == 0 && y != 0 {
                    res.push('\n');
                }
                if let Some(view) = get_view_checked(image, x, y, width, height) {
                    let view_size = (view.height() * view.width()) as f32;
                    let brightness: f32 = {
                        let sum_brightness: f32 = view.pixels().map(luma_to_brightness).sum();
                        sum_brightness / view_size
                    };
                    let index = (brightness * max_index) as usize;
                    let b = self.grayscale[index];
                    res.push(b);
                }
            }
        }
        res
    }
}

fn luma_to_brightness<T>((.., pix): (u32, u32, Luma<T>)) -> f32
where
    T: Primitive,
{
    pix.0[0].to_f32().unwrap() / 255.0
}
