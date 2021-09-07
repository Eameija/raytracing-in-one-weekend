use std::fs::File;
use std::io::BufWriter;
use std::path::Path;

pub fn output(image: Vec<u8>, width: u32, height: u32) {
    let path = Path::new(r"./image.png");
    let file = File::create(path).unwrap();
    let w = &mut BufWriter::new(file);

    let mut encoder = png::Encoder::new(w, width, height);
    encoder.set_color(png::ColorType::RGBA);
    encoder.set_depth(png::BitDepth::Eight);
    let mut writer = encoder.write_header().unwrap();

    writer.write_image_data(&image).unwrap();
}
