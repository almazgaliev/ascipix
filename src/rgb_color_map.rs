use image::{imageops::ColorMap, ImageBuffer, Pixel, Primitive, Rgb};
use std::ops::{Deref, DerefMut};

#[derive(Clone)]
pub struct PosterizationMap {
    color_thresholds: [u32; 3], // should be >= 2
}

impl Default for PosterizationMap {
    fn default() -> Self {
        Self {
            color_thresholds: [2, 2, 2],
        }
    }
}

impl PosterizationMap {
    pub fn new(color_thresholds: [u32; 3]) -> Self {
        assert!(color_thresholds.iter().all(|&x| x >= 2), "color threshold should be >= 2");
        Self { color_thresholds }
    }
}

impl ColorMap for PosterizationMap {
    type Color = Rgb<u8>;

    #[inline(always)]
    fn index_of(&self, src_color: &Self::Color) -> usize {
        let mut color: [u8; 3] = Default::default();
        for ((&channel_src, channel_dest), n) in src_color
            .channels()
            .iter()
            .zip(color.iter_mut())
            .zip(self.color_thresholds)
        {
            let n = n as f32;
            let float_color = (n * (channel_src as f32) / 255.0).round();
            let color = (float_color * 255.0 / n).round() as u8;
            *channel_dest = color;
        }

        let mut index: [u8; 8] = Default::default();
        index[0..3].clone_from_slice(&color);
        usize::from_ne_bytes(index)
    }

    #[inline(always)]
    fn lookup(&self, idx: usize) -> Option<Self::Color> {
        if idx > 0xFFFFFF {
            None
        } else {
            let mut color: [u8; 3] = Default::default();
            color.clone_from_slice(&idx.to_ne_bytes()[0..3]);
            Some(Rgb(color))
        }
    }

    /// Indicate NeuQuant implements `lookup`.
    fn has_lookup(&self) -> bool {
        true
    }

    #[inline(always)]
    fn map_color(&self, color: &mut Rgb<u8>) {
        *color = self.lookup(self.index_of(color)).unwrap();
    }
}

