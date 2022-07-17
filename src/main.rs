use clap::{arg, ArgAction, Command};
use image::{ImageBuffer, Rgb,imageops::FilterType};
use std::path::Path;

fn main() {
    // TODO add cli argument for scale
    let scale: (u32, u32) = (1, 2); // one character width height ratio in terminal
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
            arg!(-s --size)
                .id("size")
                .help("sets window size")
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
                .action(ArgAction::SetFalse),
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

    let grayscale = {
        let index = *matches.get_one::<usize>("grayscale_index").unwrap();
        let invert = *matches.get_one("invert").unwrap();

        let grayscales = ["WHyzv;\"` ", "NWyx?!`. ", "@%#*+=-:. ", "@$#*!=;:~-,. "];
        let mut grayscale: Vec<char> = grayscales[index].chars().collect();
        if invert {
            grayscale.reverse();
        }
        grayscale
    };

    let image = {
        let path = matches.get_one::<String>("input_image").unwrap();
        let image_path = Path::new(path);
        image::open(image_path).unwrap()
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
    let image = image.resize_exact(size[0] / scale.0, size[1] / scale.1, FilterType::Triangle);

    output(image.into_rgb32f(), grayscale);
}

/// prints image into stdout using grayscale characters
fn output(buffer: ImageBuffer<Rgb<f32>, Vec<f32>>, grayscale: Vec<char>) {
    let max_index = (grayscale.len() - 1) as f32;
    for (x, y, &pix) in buffer.enumerate_pixels() {
        if x == 0 && y != 0 {
            println!("");
        }
        let brightness = (pix[0] + pix[1] + pix[2]) / 3.0;
        let index = (brightness * max_index) as usize;
        let b = grayscale
            .get(index)
            .unwrap();
        print!("{}", b);
    }
    println!("");
}
