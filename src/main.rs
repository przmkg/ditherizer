extern crate image;

use image::{GrayImage, Luma};

fn main() {
    let image = image::open("image.png").unwrap();

    let mut grayscale = image.to_luma();


    let result = ditherize(&mut grayscale);

    result.save("image2.png").unwrap();
}

fn ditherize(data: &mut GrayImage) -> GrayImage {
    let (width, height) = data.dimensions();
    let mut result = data.clone();

    for (x, y, _pixel) in data.enumerate_pixels() {
        if x > width - 1 || y > height - 1 {
            continue;
        }
            let old_pixel = result.get_pixel_mut(x, y).clone(); //pixel.clone();
            let new_pixel = find_closest_palette_color(&old_pixel);
            result.put_pixel(x, y, new_pixel);
            let quant_error = old_pixel[0] as f32 - new_pixel[0] as f32;

            if x < width - 2 {
                let pixel1 = result.get_pixel_mut(x + 1, y).clone();
                result.put_pixel(x + 1, y, image::Luma([(pixel1[0] as f32 + quant_error * 7. / 16.) as u8]));
            }

            if x != 0 && y < height - 2 {
                let pixel2 = result.get_pixel_mut(x - 1, y + 1).clone();
                result.put_pixel(x.saturating_sub(1), y + 1, image::Luma([(pixel2[0] as f32 + quant_error * 3. / 16.) as u8]));
            }

            if y < height - 2 {
                let pixel3 = result.get_pixel_mut(x, y + 1).clone();
                result.put_pixel(x, y + 1, image::Luma([(pixel3[0] as f32 + quant_error * 5. / 16.) as u8]));
            }

            if x < width - 2 && y < height - 2 {
                let pixel4 = result.get_pixel_mut(x + 1, y + 1).clone();
                result.put_pixel(x + 1, y + 1, image::Luma([(pixel4[0] as f32 + quant_error * 1. / 16.) as u8]));
            }
    }

    println!("data size: {}", data.len());
    return result;
}

fn find_closest_palette_color(old_pixel: &Luma<u8>) -> Luma<u8> {
    if old_pixel[0] > 122u8 {
        image::Luma([255u8])
    } else {
        image::Luma([0u8])
    }
}

/*fn _convert_to_grayscale(data: &Vec<u8>) -> Vec<u8> {
    data.chunks(3)
        .map(|rgb| (0.299 * rgb[0] as f32 + 0.587 * rgb[1] as f32 + 0.114 * rgb[2] as f32) as u8)
        .collect::<Vec<u8>>()
}*/
