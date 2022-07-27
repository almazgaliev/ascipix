use clap::{arg, ArgAction, Command};
use image::{imageops::FilterType, open};
use opencv::{imgproc::canny, types::VectorOfu8};
use std::path::Path;

fn main() {
    // TODO add cli argument for one character's sides ratio
    let scale: (u32, u32) = (1, 2); // one character width height ratio in terminal
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
            arg!(-i --input <PATH>)
                .id("input_image")
                .help("image to convert")
                .required(true),
        )
        .arg(
            arg!(-s - -size)
                .id("size")
                .help("sets image size")
                .conflicts_with("scale")
                .number_of_values(2)
                .require_value_delimiter(true)
                .value_delimiter('x')
                .value_parser(clap::value_parser!(u32)),
        )
        .arg(
            arg!(--scale <VALUE>)
                .id("scale")
                .required(false)
                .default_value("100")
                .help("scale original image in percents")
                .value_parser(clap::value_parser!(u32)),
        )
        .arg(
            arg!(--invert)
                .id("invert")
                .help("invert colors")
                .required(false)
                .action(ArgAction::SetFalse),
        )
        .arg(
            arg!(-g --grayscaleindex <VALUE>)
                .help("string of character to use in art")
                .id("grayscale_index")
                .required(false)
                .default_value("0")
                .value_parser(clap::value_parser!(usize)),
        )
        .arg(
            arg!(-e - -edges)
                .help("use canny to extract edges")
                .id("edge_filter")
                .action(ArgAction::SetTrue),
        )
        .arg(
            arg!(-l --low <VALUE>)
                .id("low")
                .required(false)
                .default_value("50")
                .help("low value")
                .value_parser(clap::value_parser!(f64)),
        )
        .arg(
            arg!(-t --threshold <VALUE>)
                .id("threshold")
                .help("threshold values")
                .number_of_values(2)
                .required(false)
                .default_values(&["50", "200"])
                .value_parser(clap::value_parser!(f64)),
        )
        .arg(
            arg!(-l - -l2gradient)
                .id("l2")
                .required(false)
                .help("use l2gradient")
                .action(ArgAction::SetTrue),
        )
        .arg(
            arg!(-a --aperture <VALUE>)
                .id("aperture")
                .required(false)
                .default_value("3")
                .value_parser(clap::value_parser!(i32)),
        )
        .get_matches();

    let (threshold1, threshold2) = {
        let threshold: Vec<f64> = matches.get_many::<f64>("threshold").unwrap().copied().collect();
        (threshold[0], threshold[1])
    };
    let aperture_size = *matches.get_one::<i32>("aperture").unwrap();
    let l2 = *matches.get_one::<bool>("l2").unwrap();
    let in_path = Path::new(matches.get_one::<String>("input_image").unwrap());
    let index = *matches.get_one::<usize>("grayscale_index").unwrap();
    let invert = *matches.get_one("invert").unwrap();
    let use_canny = *matches.get_one::<bool>("edge_filter").unwrap();
    let size = matches.get_many("size");

    let grayscale = {
        let grayscales = ["WHyzv;\"` ", "NWyx?!`. ", "@%#*+=-:. ", "@$#*!=;:~-,. "];
        let mut grayscale: Vec<char> = grayscales[index].chars().collect();
        if invert {
            grayscale.reverse();
        }
        grayscale
    };

    let image = {
        let mut image = open(in_path).unwrap();

        let size: Vec<u32> = match size {
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
        image = image.resize_exact(size[0] / scale.0, size[1] / scale.1, FilterType::Nearest);

        if use_canny {
            let image_size = (image.width(), image.height());
            let image_in = &VectorOfu8::from_slice(&image.into_luma8().to_vec()[..]);

            let image_out = &mut VectorOfu8::with_capacity(image_in.capacity());
            for _ in 0..image_out.capacity() {
                image_out.push(0);
            }

            canny(
                image_in,
                image_out,
                threshold1,
                threshold2,
                aperture_size,
                l2,
            )
            .expect("Canny error");

            image::GrayImage::from_vec(image_size.0, image_size.1, image_out.to_vec()).unwrap()
        } else {
            image.into_luma8()
        }
    };

    let max_index = grayscale.len() - 1;

    for (x, y, &pix) in image.enumerate_pixels() {
        if x == 0 && y != 0 {
            println!("");
        }
        let brightness = pix[0] as f32 / 255.0;
        let index = (brightness * max_index as f32) as usize;
        let b = grayscale.get(index).unwrap();
        print!("{}", b);
    }
    println!("");
}
