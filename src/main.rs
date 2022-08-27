mod charify;
mod charmaps;

mod dither;
mod rgb_color_map;
use crate::charify::Charify;
use crate::charmaps::{BrailleCharMap, GrayCharMap};
use clap::{arg, ArgAction, Command};
use image::{imageops::FilterType, Luma};
use std::path::Path;

fn main() {
    // TODO add declarative argparser
    let matches = Command::new("ascipix")
        .version(env!("CARGO_PKG_VERSION"))
        .author("Galiev A. <almazgaliev99@gmail.com>")
        .about(
            r"

        _/_/                        _/            _/           
     _/    _/    _/_/_/    _/_/_/      _/_/_/        _/    _/  
    _/_/_/_/  _/_/      _/        _/  _/    _/  _/    _/_/     
   _/    _/      _/_/  _/        _/  _/    _/  _/  _/    _/    
  _/    _/  _/_/_/      _/_/_/  _/  _/_/_/    _/  _/    _/     
                                   _/                          
                                  _/                           

small tool to convert images into ascii art
        ",
        )
        .arg(
            arg!(-i --input <VALUE>)
                .help("image to convert")
                .id("input_image")
                .required(true),
        )
        .arg(
            arg!(-s --size <VALUE>)
                .id("size")
                .help("sets image size")
                .conflicts_with("scale")
                .number_of_values(2)
                .require_value_delimiter(true)
                .value_delimiter('x')
                .value_parser(clap::value_parser!(u32)),
        )
        .arg(
            arg!(--scale <NTIMES>)
                .id("scale")
                .required(false)
                .default_value("100")
                .help("scale original image in percents")
                .value_parser(clap::value_parser!(u32)),
        )
        .arg(
            arg!(--invert)
                .help("invert colors")
                .id("invert")
                .required(false)
                .action(ArgAction::SetTrue),
        )
        .arg(
            arg!(--dither)
                .help("use dithered image")
                .id("dither")
                .required(false)
                .action(ArgAction::SetTrue),
        )
        .arg(
            arg!(-g --grayscale <VALUE>)
                .help("string of character to use in art")
                .id("grayscale_index")
                .required(false)
                .default_value("0")
                .value_parser(clap::value_parser!(usize)),
        )
        .get_matches();

    let invert: bool = *matches.get_one("invert").unwrap();
    let image = {
        let path = matches.get_one::<String>("input_image").unwrap();
        let image_path = Path::new(path);
        let mut image = image::open(image_path).unwrap();
        if invert {
            image.invert();
        }
        image
    };

    let size: Vec<u32> = match matches.get_many("size") {
        Some(v) => v.copied().collect(),
        None => match matches.get_one::<u32>("scale") {
            Some(&scale) => {
                let (mut width, mut height) = (image.width() as f64, image.height() as f64);
                width *= scale as f64 / 100.0;
                height *= scale as f64 / 100.0;
                vec![width as u32, height as u32]
            }
            None => vec![image.width(), image.height()],
        },
    };

    let image = image
        .resize_exact(size[0], size[1], FilterType::Nearest)
        .into_luma8();

    let dither: bool = *matches.get_one("dither").unwrap();

    let grayscale: Box<dyn Charify<Luma<u8>, u8, Vec<u8>>> = {
        if dither {
            Box::new(BrailleCharMap::default())
        } else {
            let index = *matches.get_one::<usize>("grayscale_index").unwrap();
            Box::new(GrayCharMap::from_existing(index))
        }
    };
    let res = grayscale.charify(&image);
    println!("{}", res);
}
