use image::load;
use std::{fs::File, io::BufReader, path::Path};
static SCALE_X: usize = 1;
static SCALE_Y: usize = 2;

fn main() {
    //TODO write cli with clap
    //TODO add custom character sets

    let chars: &str = "$@B%8&WM#*oahkbdpqwmZO0QLCJUYXzcvunxrjft/\\|()1{}[]?-_+~<>i!lI;:,\"^`'.";
    let buffer = {
        let image_path = Path::new("./th-2196793002.jpg");
        let reader = File::open(image_path).unwrap();
        load(BufReader::new(reader), image::ImageFormat::Jpeg)
            .unwrap()
            .into_rgb32f()
    };

    for (x, _, pix) in buffer.enumerate_pixels() {
        if x == 0 {
            println!("");
        }
        let index = ((pix[0] + pix[1] + pix[2]) / 3.0 * chars.len() as f32) as usize;
        print!("{}",chars.chars().rev().nth(index).unwrap());
    }
}
