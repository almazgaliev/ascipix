use image::{ImageBuffer, Pixel};

pub trait Charify<'a, P, T, C>
where
    P: Pixel<Subpixel = T>
{
    fn charify(&self, image: &'a ImageBuffer<P, C>) -> String;
}
