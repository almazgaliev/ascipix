macro_rules! do_dithering(
    ($map:expr, $image:expr, $err:expr, $x:expr, $y:expr) => (
        {
            let old_pixel = $image[($x, $y)];
            let new_pixel = $image.get_pixel_mut($x, $y);
            $map.map_color(new_pixel);
            for ((e, &old), &new) in $err.iter_mut()
                                        .zip(old_pixel.channels().iter())
                                        .zip(new_pixel.channels().iter())
            {
                *e = <i16 as From<_>>::from(old) - <i16 as From<_>>::from(new)
            }
        }
    )
);

use image::{
    imageops::{dither, ColorMap},
    ImageBuffer, Pixel,
};

pub trait Dither {
    fn dither<Pix, T>(&self, image: &mut ImageBuffer<Pix, Vec<u8>>, color_map: &T)
    where
        Pix: Pixel<Subpixel = u8> + 'static,
        T: ColorMap<Color = Pix>;
}

// TODO reimplement dithering with more polymorphyc function
pub struct FloydSteinberg;

impl Dither for FloydSteinberg {
    fn dither<Pix, T>(&self, image: &mut ImageBuffer<Pix, Vec<u8>>, color_map: &T)
    where
        Pix: Pixel<Subpixel = u8> + 'static,
        T: ColorMap<Color = Pix>,
    {
        dither(image, color_map);
    }
}

pub struct Bayer;

impl Dither for Bayer {
    fn dither<Pix, T>(&self, image: &mut ImageBuffer<Pix, Vec<u8>>, color_map: &T)
    where
        Pix: Pixel<Subpixel = u8> + 'static,
        T: ColorMap<Color = Pix>,
    {
        let image: &mut ImageBuffer<Pix, Vec<u8>> = image;
        let color_map = color_map;
        let (width, height) = image.dimensions();
        let mut err: [i16; 3] = [0; 3];
        for y in 0..height - 1 {
            let x = 0;

            do_dithering!(color_map, image, err, x, y);
            diffuse_err(image.get_pixel_mut(x + 1, y), err, 7);
            diffuse_err(image.get_pixel_mut(x, y + 1), err, 5);
            diffuse_err(image.get_pixel_mut(x + 1, y + 1), err, 1);

            for x in 1..width - 1 {
                do_dithering!(color_map, image, err, x, y);
                diffuse_err(image.get_pixel_mut(x + 1, y), err, 7);
                diffuse_err(image.get_pixel_mut(x - 1, y + 1), err, 3);
                diffuse_err(image.get_pixel_mut(x, y + 1), err, 5);
                diffuse_err(image.get_pixel_mut(x + 1, y + 1), err, 1);
            }
            let x = width - 1;

            do_dithering!(color_map, image, err, x, y);
            diffuse_err(image.get_pixel_mut(x - 1, y + 1), err, 3);
            diffuse_err(image.get_pixel_mut(x, y + 1), err, 5);
        }
        let y = height - 1;
        let x = 0;

        do_dithering!(color_map, image, err, x, y);
        diffuse_err(image.get_pixel_mut(x + 1, y), err, 7);

        for x in 1..width - 1 {
            do_dithering!(color_map, image, err, x, y);
            diffuse_err(image.get_pixel_mut(x + 1, y), err, 7);
        }
        let x = width - 1;
        do_dithering!(color_map, image, err, x, y);
    }
}

fn diffuse_err<P: Pixel<Subpixel = u8>>(pixel: &mut P, error: [i16; 3], factor: i16) {
    for (e, c) in error.iter().zip(pixel.channels_mut().iter_mut()) {
        *c = match <i16 as From<_>>::from(*c) + e * factor / 16 {
            val if val < 0 => 0,
            val if val > 0xFF => 0xFF,
            val => val as u8,
        }
    }
}
