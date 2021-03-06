use std::fs::File;
use std::io::Read;
use std::path::{Path, PathBuf};

use ::image;

use dct;
use quantize;
use color_space;

use flate2::read::ZlibDecoder;

use protobuf;
use compressed_image::compressed_image;

#[derive(PartialEq)]
enum ImageType {
    Unknown,
    MSCA,
    Image(image::ImageFormat)
}


pub fn uncompress_file(input_filename: &Path) {
    if let Some(file_stem) = input_filename.file_stem() {
        if let Some(file_container) = input_filename.parent() {
            let mut output_filename = PathBuf::from(&file_container);
            output_filename.push(file_stem);
            output_filename.set_extension("png");

            uncompress_file_to_output(input_filename, output_filename.as_path());
        } else {
            panic!("Invalid input filename: Could not automatically determine output file")
        }
    } else {
        panic!("Invalid input filename: Could not automatically determine output file")
    }
}

pub fn uncompress_file_to_output(input_filename: &Path, output_filename: &Path) {

    let input_format = determine_image_format(input_filename);
    let output_format = determine_image_format(output_filename);

    assert!(input_format == ImageType::MSCA, "Input file for uncompression must be in MSCA format");

    match output_format {
        ImageType::Image(format) => {
            let mut input_file = File::open(&Path::new(&input_filename)).unwrap();
            let mut output_file = File::create(&Path::new(&output_filename)).unwrap();

            uncompress(&mut input_file, &mut output_file, format);
        }
        _ => panic!("Output file for uncompression must be PNG or JPEG format")
    }
}

fn determine_image_format(filename: &Path) -> ImageType {
    if let Some(extension) = filename.extension() {
        if let Some(extension_str) = extension.to_str() {
            let extension_lowercase = extension_str.to_lowercase();

            match extension_lowercase.as_ref() {
                "png" => { return ImageType::Image(image::PNG); },
                "jpg" => { return ImageType::Image(image::JPEG); },
                "jpeg" => { return ImageType::Image(image::JPEG); }
                "msca" => { return ImageType::MSCA; }
                _ => { return ImageType::Unknown; }
            }
        }
    }

    ImageType::Unknown
}

fn uncompress(input: &mut File, output: &mut File, output_format: image::ImageFormat) {

    // uncompress data from input file
    let mut serialized = Vec::new();
    ZlibDecoder::new(input).read_to_end(&mut serialized).unwrap();

    // deserialize the uncompressed data
    let mut deserialized: compressed_image = protobuf::parse_from_bytes(serialized.as_slice())
        .unwrap();

    let width = deserialized.get_width();
    let height = deserialized.get_height();

    //perform the decompression
    let uncompressed_red = uncompress_color_channel(width as usize, height as usize, deserialized.take_red());
    let uncompressed_green = uncompress_color_channel(width as usize, height as usize, deserialized.take_green());
    let uncompressed_blue = uncompress_color_channel(width as usize, height as usize, deserialized.take_blue());

    // create the output image and load the data in
    let mut output_image = image::ImageBuffer::new(width, height);

    for x in 0..width {
        for y in 0..height {
            let (r,g,b) = color_space::ycbcr_to_rgb(
                uncompressed_red[(x + y * width) as usize],
                uncompressed_green[(x + y * width) as usize],
                uncompressed_blue[(x + y * width) as usize]
                );

            let pixel = image::Rgba([r,g,b,255]);

            output_image.put_pixel(x,y,pixel);
        }
    }

    let _ = image::ImageRgba8(output_image).save(output, output_format);
}

fn uncompress_color_channel(width: usize, height: usize, quantized_data: Vec<i32>) -> Vec<f32> {

    let mut decoded_data = quantize::decode(width, height, &quantized_data);

    // run a 2d DCT3 on the input data
    dct::dct3_2d(width, height, &mut decoded_data);

    // finally, scale the result by 4 / n^2 to get the original image data (or what's left of it)
    let result_scale = 4_f32 / (decoded_data.len()) as f32;
    for item in &mut decoded_data {
        *item *= result_scale;
    }
    
    decoded_data
}
