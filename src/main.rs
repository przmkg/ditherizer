use std::{fs::File, io::BufWriter, path::Path};

extern crate png;

const WIDTH: usize = 1174;
const HEIGHT: usize = 866;
const LINE_SIZE: usize = 3522;

fn main() {
    let (_reader, buf) = load_image();

    let path = Path::new(r"image2.png");
    let file = File::create(path).unwrap();
    let ref mut w = BufWriter::new(file);

    let mut grayscale = convert_to_grayscale(&buf);
    ditherize(&mut grayscale, WIDTH, HEIGHT, LINE_SIZE / 3);

    let mut encoder = png::Encoder::new(w, WIDTH as u32, HEIGHT as u32);
    encoder.set_color(png::ColorType::Grayscale);
    encoder.set_depth(png::BitDepth::One);
    let mut writer = encoder.write_header().unwrap();

    // writer.write_image_data(&grayscale[0..1015818]).unwrap();
    writer.write_image_data(&grayscale).unwrap();
}

fn load_image() -> (png::Reader<File>, Vec<u8>) {
    let decoder = png::Decoder::new(File::open("image.png").unwrap());
    let (info, mut reader) = decoder.read_info().unwrap();
    println!("{:?}", info);

    let mut buf = vec![0; info.buffer_size()];
    reader.next_frame(&mut buf).unwrap();

    println!("Size: {}", buf.len());
    println!("Width: {}\tHeight: {}", info.width, info.height);

    return (reader, buf);
}

fn ditherize(data: &mut Vec<u8>, width: usize, height: usize, line_size: usize) {
    for y in 0..height - 1 {
        for x in 0..width - 1 {
            let pixel_pos = (y * line_size) + x;
            let old_pixel = data[pixel_pos];
            let new_pixel = find_closest_palette_color(old_pixel);
            data[pixel_pos] = new_pixel;
            let quant_error: i32 = old_pixel as i32 - new_pixel as i32;

            let pixel1 = data[(y * line_size) + x + 1];
            let pixel2 = data[((y + 1) * line_size) + x - 1];
            let pixel3 = data[((y + 1) * line_size) + x];
            let pixel4 = data[((y + 1) * line_size) + x + 1];

            // println!("pixel1 {}\tpixel2 {}\tpixel3 {}\tpixel4 {}", pixel1, pixel2, pixel3, pixel4);

            let new_pixel1 = (pixel1 as i32 + quant_error) * 7 / 16;
            let new_pixel2 = (pixel2 as i32 + quant_error) * 3 / 16;
            let new_pixel3 = (pixel3 as i32 + quant_error) * 5 / 16;
            let new_pixel4 = (pixel4 as i32 + quant_error) / 16;

            // println!("pixel1 {}\tpixel2 {}\tpixel3 {}\tpixel4 {}", new_pixel1, new_pixel2, new_pixel3, new_pixel4);

            data[(y * line_size) + x + 1] = new_pixel1 as u8;
            data[((y + 1) * line_size) + x - 1] = new_pixel2 as u8;
            data[((y + 1) * line_size) + x] = new_pixel3 as u8;
            data[((y + 1) * line_size) + x + 1] = new_pixel4 as u8;
            // println!("x: {} y: {}", x, y);
            // println!("pixel pos: {}", pixel_pos);
        }
    }

    println!("data size: {}", data.len());
}

fn find_closest_palette_color(old_pixel: u8) -> u8 {
    if old_pixel > 122u8 {
        255
    } else {
        0
    }
}

fn convert_to_grayscale(data: &Vec<u8>) -> Vec<u8> {
    data.chunks(3)
        .map(|rgb| (0.299 * rgb[0] as f32 + 0.587 * rgb[1] as f32 + 0.114 * rgb[2] as f32) as u8)
        .collect::<Vec<u8>>()
}
