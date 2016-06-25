use std::fs::File;
use std::io::BufReader;
use std::io::Write;
use std::path::{Path, PathBuf};

use ::image;
use ::image::GenericImage;

use dct;
use quantize;

use compressed_image;
use protobuf::Message;

use flate2::Compression;
use flate2::write::ZlibEncoder;





pub fn compress_file(input_filename: &Path) {
    let file_stem = match input_filename.file_stem() {
        Some(stem) => stem,
        None => panic!("Invalid input filename: Could not automatically determine output file"),
    };

    let file_container = match input_filename.parent() {
        Some(result) => result,
        None => {
            panic!("Invalid input filename: Could not automatically determine the output file \
                    directory")
        }
    };

    let mut output_filename = PathBuf::from(&file_container);
    output_filename.push(file_stem);
    output_filename.set_extension("msca");

    compress_file_to_output(input_filename, output_filename.as_path());
}

pub fn compress_file_to_output(input_filename: &Path, output_filename: &Path) {
    // verify that the filename extensions match what we expect
    if let Some(extension) = input_filename.extension() {
        assert!(extension == "png",
                "Input file for compression must be 'PNG'")
    } else {
        panic!("Input file for compression must be 'msca'")
    }

    if let Some(extension) = input_filename.extension() {
        assert!(extension == "msca",
                "Input file for compression must be 'msca'")
    } else {
        panic!("Output file for compression must be msca")
    }

    let input_file = File::open(&Path::new(&input_filename)).unwrap();
    let mut output_file = File::create(&Path::new(&output_filename)).unwrap();

    compress(&input_file, &mut output_file);
}

fn compress(input: &File, output: &mut File) {
    let input_image = image::load(BufReader::new(input), image::PNG).unwrap();

    let (width, height) = input_image.dimensions();

    let mut red_channel: Vec<f32> = Vec::with_capacity(width as usize * height as usize);
    let mut green_channel: Vec<f32> = Vec::with_capacity(width as usize * height as usize);
    let mut blue_channel: Vec<f32> = Vec::with_capacity(width as usize * height as usize);
    let mut alpha_channel: Vec<f32> = Vec::with_capacity(width as usize * height as usize);

    // split the color data into channels
    for y in 0..height {
        for x in 0..width {
            let pixel = input_image.get_pixel(x, y);

            red_channel.push((pixel[0] as f32) - 128_f32);
            green_channel.push((pixel[1] as f32) - 128_f32);
            blue_channel.push((pixel[2] as f32) - 128_f32);
            alpha_channel.push((pixel[3] as f32) - 128_f32);
        }
    }

    let mut serializer = compressed_image::compressed_image::new();
    serializer.set_width(width);
    serializer.set_height(height);

    // compress the data and put it directly into the serializer
    serializer.set_red(compress_color_channel(width as usize, height as usize, red_channel));
    serializer.set_green(compress_color_channel(width as usize, height as usize, green_channel));
    serializer.set_blue(compress_color_channel(width as usize, height as usize, blue_channel));
    serializer.set_alpha(compress_color_channel(width as usize, height as usize, alpha_channel));

    let serialized_bytes = serializer.write_to_bytes().unwrap();

    // losslessly compress the serialized data
    let mut enc = ZlibEncoder::new(output, Compression::Default);
    let mut written = 0;

    while written < serialized_bytes.len() {
        written += enc.write(&serialized_bytes[written..serialized_bytes.len()]).unwrap();
    }
    let _ = enc.finish();
}

fn compress_color_channel(width: usize,
                          height: usize,
                          mut uncompressed_channel_data: Vec<f32>)
                          -> Vec<i32> {
    dct::dct2_2d(width, height, &mut uncompressed_channel_data);
    quantize::encode(width, height, &uncompressed_channel_data)
}
