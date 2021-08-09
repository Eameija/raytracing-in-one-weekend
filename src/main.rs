use std::fs::File;
use std::io::BufWriter;
use std::path::Path;

const WIDTH: u16 = 256;
const HEIGHT: u16 = 256;

fn main() {
    let path = Path::new(r"./image.png");
    let file = File::create(path).unwrap();
    let ref mut w = BufWriter::new(file);

    let mut encoder = png::Encoder::new(w, u32::from(WIDTH), u32::from(HEIGHT));
    encoder.set_color(png::ColorType::RGBA);
    encoder.set_depth(png::BitDepth::Eight);
    let mut writer = encoder.write_header().unwrap();

    let mut pixels: Vec<u8> = vec![];

    for y in 0..HEIGHT {
        println!("Remaining lines: {}", HEIGHT - y);
        for x in 0..WIDTH {
            let xi = x as f64 / (WIDTH - 1) as f64;
            let yi = y as f64 / (HEIGHT - 1) as f64;

            let red = (255.0 * xi) as u8;
            let green = (255.0 * yi) as u8;

            pixels.push(red);
            pixels.push(green);
            pixels.push(32);
            pixels.push(255);
        }
    }
    writer.write_image_data(&pixels).unwrap();
}
