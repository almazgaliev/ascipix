use clap::{arg, ArgAction, Command};
use image::{load, ImageBuffer, Rgb};
use std::{fs::File, io::BufReader, path::Path};

static SCALE_X: usize = 1;
static SCALE_Y: usize = 2;



fn read_jpg(image_path: &Path) -> ImageBuffer<Rgb<f32>, Vec<f32>> {
    if !image_path.exists() {
        panic!("file does not exists: {}", image_path.to_str().unwrap());
    }
    let reader = File::open(image_path).unwrap();
    load(BufReader::new(reader), image::ImageFormat::Jpeg)
        .unwrap()
        .into_rgb32f()
}

fn main() {
    let matches = Command::new("ascipix")
        .version(env!("CARGO_PKG_VERSION"))
        .author("Galiev A. <almazgaliev99@gmail.com>")
        .about("converts image into ascii art")
        .arg(
            arg!(-i --input <VALUE>)
                .help("image to convert")
                .id("input_image")
                .required(true),
        )
        .arg(
            arg!(-r --reversed)
                .help("inverse colors")
                .id("rev")
                .required(false)
                .action(ArgAction::SetFalse),
        )
        .get_matches();

    //TODO add custom character sets

    let grayscale: &str = "@$#*!=;:~-,. ";
    let mut grayscale: Vec<char> = grayscale.chars().collect();

    let max_index = (grayscale.len() - 1) as f32;
    let buffer = {
        let path = matches.get_one::<String>("input_image").unwrap();
        let image_path = Path::new(path);
        read_jpg(image_path)
    };

    if *matches.get_one::<bool>("rev").unwrap() {
        grayscale.reverse();
    }

    for (x, _, pix) in buffer.enumerate_pixels() {
        if x == 0 {
            println!("");
        }
        let index = ((pix[0] + pix[1] + pix[2]) / 3.0 * max_index) as usize;
        let b = grayscale.get(index).unwrap();
        print!("{} ", b);
    }
    println!("");
}
