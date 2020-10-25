extern crate image;

use image::{GrayImage, Luma};

fn main() {
    let args: Vec<String> = std::env::args().collect();

    if args.len() != 3 {
        println!("Wrong arguments.");
        println!("Usage: ditherizer input.png output.png");
        return;
    }

    let input = &args[1];
    let output = &args[2];

    let image = image::open(input).unwrap();

    let mut grayscale = image.to_luma();

    ditherize(&mut grayscale);

    grayscale.save(output).unwrap();
}

fn ditherize(data: &mut GrayImage) {
    let (width, height) = data.dimensions();

    for y in 0..height {
        for x in 0..width {
            let old_pixel = data.get_pixel(x, y).clone(); //pixel.clone();
            let new_pixel = find_closest_palette_color(&old_pixel);
            data.put_pixel(x, y, new_pixel);
            let quant_error = old_pixel[0] as f32 - new_pixel[0] as f32;

            if x < width - 2 {
                let pixel1 = data.get_pixel(x + 1, y).clone();
                data.put_pixel(
                    x + 1,
                    y,
                    image::Luma([(pixel1[0] as f32 + quant_error * 7. / 16.) as u8]),
                );
            }

            if x != 0 && y < height - 2 {
                let pixel2 = data.get_pixel(x - 1, y + 1).clone();
                data.put_pixel(
                    x.saturating_sub(1),
                    y + 1,
                    image::Luma([(pixel2[0] as f32 + quant_error * 3. / 16.) as u8]),
                );
            }

            if y < height - 2 {
                let pixel3 = data.get_pixel(x, y + 1).clone();
                data.put_pixel(
                    x,
                    y + 1,
                    image::Luma([(pixel3[0] as f32 + quant_error * 5. / 16.) as u8]),
                );
            }

            if x < width - 2 && y < height - 2 {
                let pixel4 = data.get_pixel(x + 1, y + 1).clone();
                data.put_pixel(
                    x + 1,
                    y + 1,
                    image::Luma([(pixel4[0] as f32 + quant_error * 1. / 16.) as u8]),
                );
            }
        }
    }
}

fn find_closest_palette_color(old_pixel: &Luma<u8>) -> Luma<u8> {
    if old_pixel[0] > 122u8 {
        image::Luma([255u8])
    } else {
        image::Luma([0u8])
    }
}
