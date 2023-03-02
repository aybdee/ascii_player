use crate::mp4;
use colored::Colorize;
use image::{imageops::FilterType::Gaussian, GenericImageView, Rgb, RgbImage};
use std::error::Error;

#[derive(Clone)]
pub struct AsciiPixel {
    chars: String,
    rgb: Rgb<u8>,
}

pub struct AsciiImage {
    image: Vec<Vec<AsciiPixel>>,
}

impl AsciiImage {
    fn new(width: u32, height: u32) -> Self {
        let image = vec![
            vec![
                AsciiPixel {
                    chars: String::from(' '),
                    rgb: Rgb::from([255_u8, 255, 255])
                };
                (width + 1) as usize
            ];
            (height + 1) as usize
        ];
        Self { image }
    }

    fn render_ascii(&self) {
        for line in self.image.iter() {
            for pixel in line.iter() {
                let repr =
                    format!("{}", pixel.chars).truecolor(pixel.rgb[0], pixel.rgb[1], pixel.rgb[2]);
                print!("{} ", repr);
            }
            print!("\n");
        }
    }
}

fn to_ascii(img: image::RgbImage) -> AsciiImage {
    let ascii_density_map = " `.-':_,^=;><+!rc*/z?sLTv)J7(|Fi{C}fI31tlu[neoZ5Yxjya]2ESwqkP6h9d4VpOGbUAKXHm8RD#$Bg0MNWQ%&@";

    let ascii_vec: Vec<String> = ascii_density_map.chars().map(|x| x.to_string()).collect();
    let graysscale_map = |grayscale_value: u8, density_range: usize| {
        let partition_size = (255_u8 as f32 / density_range as f32).ceil();
        (grayscale_value as f32 / partition_size).ceil() as u8
    };
    let (width, height) = img.dimensions();
    let mut ascii_image = AsciiImage::new(width, height);
    for (x, y, pixel) in img.enumerate_pixels() {
        let pixels: u16 = pixel.0.iter().map(|x| x.clone() as u16).sum();
        let pixel_density = (pixels as f32 / 3 as f32) as u8;
        let ascii_repr = graysscale_map(pixel_density, ascii_vec.len());
        ascii_image.image[y as usize][x as usize] = AsciiPixel {
            chars: ascii_vec[ascii_repr as usize].clone(),
            rgb: pixel.clone(),
        }
    }
    ascii_image
}

fn get_image(path: &str, width: u32, height: u32) -> Result<RgbImage, Box<dyn Error>> {
    let mut img = image::open(path)?;
    img = img.resize(width, height, Gaussian);
    Ok(img.to_rgb8())
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn image_loads() {
        let image = get_image("./test_image.png", 50, 50).unwrap();
        let ascii_image = to_ascii(image);
        ascii_image.render_ascii();
    }
}
