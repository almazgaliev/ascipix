use crate::charify::Charify;
use crate::dither::{Dither, FloydSteinberg};
use image::imageops::colorops::BiLevel;
use image::{GenericImageView, ImageBuffer, Luma, Pixel};

use super::utils::view_checked;

pub struct BrailleCharMap<D: Dither> {
    ditherer: D,
}

impl Default for BrailleCharMap<FloydSteinberg> {
    fn default() -> Self {
        Self {
            ditherer: FloydSteinberg,
        }
    }
}

impl<D> BrailleCharMap<D>
where
    D: Dither,
{
    pub fn new(d: D) -> Self {
        Self { ditherer: d }
    }
}

impl<'a, D: Dither> Charify<'a, Luma<u8>, u8, Vec<u8>> for BrailleCharMap<D> {
    fn charify(&self, image: &ImageBuffer<Luma<u8>, Vec<u8>>) -> String {
        let width: u32 = 2;
        let height: u32 = 4;

        let image = &mut image.clone();
        self.ditherer.dither(image, &BiLevel);

        let mut res = String::with_capacity(((image.width() + 1) * image.height()) as usize);

        for y in (0..image.height() - 1).step_by(height as usize) {
            for x in (0..image.width()).step_by(width as usize) {
                if x == 0 && y != 0 {
                    res.push('\n');
                }
                let mut chr = '_';
                if let Some(view) = view_checked(image, x, y, width, height) {
                    chr = to_braille(view.pixels().map(luma_to_brightness));
                }
                res.push(chr);
            }
        }
        res
    }
}

fn luma_to_brightness((x, y, pix): (u32, u32, Luma<u8>)) -> (u32, u32, u32) {
    // pix.channels().iter().scan();
    (x, y, (pix.0[0] as f32 / 255.0).round() as u32)
}

#[inline(always)]
fn to_braille(pixels: impl Iterator<Item = (u32, u32, u32)>) -> char {
    let mut index: u32 = 0;
    for (y, x, point) in pixels {
        index |= point << bit_shift((x, y));
    }
    // SAFETY: this is safe cos max index is 7 bits left and 111_1111 = 255
    unsafe { char::from_u32_unchecked(0x2800 + index) } // braille in range 0x2800..0x2800 + 256
}

#[inline(always)]
fn bit_shift(coords: (u32, u32)) -> u32 {
    match coords {
        (0, 0) => 0,
        (0, 1) => 1,
        (1, 0) => 2,
        (1, 1) => 3,
        (2, 0) => 4,
        (2, 1) => 5,
        (3, 0) => 6,
        (3, 1) => 7,
        _ => panic!("braille with 8 dots only uses 8 bits, if you see this message then subimage view is too big"),
    }
}

#[test]
fn to_braille_test() {
    let s = [
        '⠀', '⠁', '⠂', '⠃', '⠄', '⠅', '⠆', '⠇', '⠈', '⠉', '⠊', '⠋', '⠌', '⠍', '⠎', '⠏', '⠐', '⠑',
        '⠒', '⠓', '⠔', '⠕', '⠖', '⠗', '⠘', '⠙', '⠚', '⠛', '⠜', '⠝', '⠞', '⠟', '⠠', '⠡', '⠢', '⠣',
        '⠤', '⠥', '⠦', '⠧', '⠨', '⠩', '⠪', '⠫', '⠬', '⠭', '⠮', '⠯', '⠰', '⠱', '⠲', '⠳', '⠴', '⠵',
        '⠶', '⠷', '⠸', '⠹', '⠺', '⠻', '⠼', '⠽', '⠾', '⠿', '⡀', '⡁', '⡂', '⡃', '⡄', '⡅', '⡆', '⡇',
        '⡈', '⡉', '⡊', '⡋', '⡌', '⡍', '⡎', '⡏', '⡐', '⡑', '⡒', '⡓', '⡔', '⡕', '⡖', '⡗', '⡘', '⡙',
        '⡚', '⡛', '⡜', '⡝', '⡞', '⡟', '⡠', '⡡', '⡢', '⡣', '⡤', '⡥', '⡦', '⡧', '⡨', '⡩', '⡪', '⡫',
        '⡬', '⡭', '⡮', '⡯', '⡰', '⡱', '⡲', '⡳', '⡴', '⡵', '⡶', '⡷', '⡸', '⡹', '⡺', '⡻', '⡼', '⡽',
        '⡾', '⡿', '⢀', '⢁', '⢂', '⢃', '⢄', '⢅', '⢆', '⢇', '⢈', '⢉', '⢊', '⢋', '⢌', '⢍', '⢎', '⢏',
        '⢐', '⢑', '⢒', '⢓', '⢔', '⢕', '⢖', '⢗', '⢘', '⢙', '⢚', '⢛', '⢜', '⢝', '⢞', '⢟', '⢠', '⢡',
        '⢢', '⢣', '⢤', '⢥', '⢦', '⢧', '⢨', '⢩', '⢪', '⢫', '⢬', '⢭', '⢮', '⢯', '⢰', '⢱', '⢲', '⢳',
        '⢴', '⢵', '⢶', '⢷', '⢸', '⢹', '⢺', '⢻', '⢼', '⢽', '⢾', '⢿', '⣀', '⣁', '⣂', '⣃', '⣄', '⣅',
        '⣆', '⣇', '⣈', '⣉', '⣊', '⣋', '⣌', '⣍', '⣎', '⣏', '⣐', '⣑', '⣒', '⣓', '⣔', '⣕', '⣖', '⣗',
        '⣘', '⣙', '⣚', '⣛', '⣜', '⣝', '⣞', '⣟', '⣠', '⣡', '⣢', '⣣', '⣤', '⣥', '⣦', '⣧', '⣨', '⣩',
        '⣪', '⣫', '⣬', '⣭', '⣮', '⣯', '⣰', '⣱', '⣲', '⣳', '⣴', '⣵', '⣶', '⣷', '⣸', '⣹', '⣺', '⣻',
        '⣼', '⣽', '⣾', '⣿',
    ];
    for num in 0..255 {
        let buf = bin(num).iter().map(|x| x * 255).collect();
        let image = &image::ImageBuffer::from_vec(2, 4, buf).unwrap();
        let t = image.view(0, 0, 2, 4);
        let ch1 = to_braille(t.pixels().map(luma_to_brightness));
        let ch2 = s[num as usize];
        assert!(ch1 == ch2, "'{}' was not equal to '{}'", ch1, ch2);
    }
}

fn bin(mut num: u8) -> Vec<u8> {
    let mut buf = vec![0, 0, 0, 0, 0, 0, 0, 0];
    let mut i = 0;
    while num != 0 {
        buf[i] = num % 2u8;
        i += 1;
        num /= 2;
    }
    return buf;
}
