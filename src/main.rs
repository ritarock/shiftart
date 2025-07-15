use anyhow::Result;
use clap::Parser;
use image::{DynamicImage, RgbaImage};

#[derive(Parser)]
struct Args {
    input: String,
    output: String,

    #[arg(short, long, default_value = "5")]
    offset: u32,
}

fn shift_channel(img: &RgbaImage, dx: i32, dy: i32, channel_index: usize) -> RgbaImage {
    let (width, height) = img.dimensions();
    let mut result = img.clone();

    for y in 0..height {
        for x in 0..width {
            let sx = x as i32 - dx;
            let sy = y as i32 - dy;

            if sx >= 0 && sy >= 0 && sx < width as i32 && sy < height as i32 {
                let source_pixel = img.get_pixel(sx as u32, sy as u32);
                let pixel = result.get_pixel_mut(x, y);
                pixel[channel_index] = source_pixel[channel_index];
            }
        }
    }

    result
}

fn main() -> Result<()> {
    let args = Args::parse();

    let img = image::open(&args.input)?.to_rgba8();
    let r_shifted = shift_channel(&img, args.offset as i32, 0, 0);
    let g_shifted = shift_channel(&r_shifted, 0, args.offset as i32, 1);
    let b_shifted = shift_channel(&g_shifted, -(args.offset as i32), 0, 2);

    let rgb_img = DynamicImage::ImageRgba8(b_shifted).into_rgb8();
    rgb_img.save(&args.output)?;
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use image::{Rgba};

    #[test]
    fn test_shift_channel_red_right() {
        let mut img = RgbaImage::new(3, 1);
        img.put_pixel(0, 0, Rgba([10, 0, 0, 255]));
        img.put_pixel(1, 0, Rgba([20, 0, 0, 255]));
        img.put_pixel(2, 0, Rgba([30, 0, 0, 255]));

        let shifted = shift_channel(&img, 1, 0, 0);

        assert_eq!(shifted.get_pixel(1, 0)[0], 10);
        assert_eq!(shifted.get_pixel(2, 0)[0], 20);
    }

    #[test]
    fn test_shift_channel_blue_down() {
        let mut img = RgbaImage::new(1, 3);
        img.put_pixel(0, 0, Rgba([0, 0, 10, 255]));
        img.put_pixel(0, 1, Rgba([0, 0, 20, 255]));
        img.put_pixel(0, 2, Rgba([0, 0, 30, 255]));

        let shifted = shift_channel(&img, 0, 1, 2);

        assert_eq!(shifted.get_pixel(0, 1)[2], 10);
        assert_eq!(shifted.get_pixel(0, 2)[2], 20);
    }
}
