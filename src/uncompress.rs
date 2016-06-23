use std::fs::File;
use std::io::Read;
use std::path::{Path, PathBuf};

use ::image;

use dct;
use quantize;

use flate2::read::ZlibDecoder;

use protobuf;
use compressed_image::compressed_image;


pub fn uncompress_file(input_filename: &Path) {
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
    output_filename.set_extension("png");

    uncompress_file_to_output(input_filename, output_filename.as_path());
}

pub fn uncompress_file_to_output(input_filename: &Path, output_filename: &Path) {
    // verify that the filename extensions match what we expect
    match input_filename.extension() {
        Some(ext) => assert!(ext == "msca", "Input file for uncompression must be 'msca'"),
        _ => {}
    }
    match output_filename.extension() {
        Some(ext) => assert!(ext == "png", "Output file for uncompression must be PNG"),
        _ => {}
    }

    let mut input_file = File::open(&Path::new(&input_filename)).unwrap();
    let mut output_file = File::create(&Path::new(&output_filename)).unwrap();

    uncompress(&mut input_file, &mut output_file);
}

fn uncompress(input: &mut File, output: &mut File) {

    // uncompress data from input file
    let mut serialized = Vec::new();
    ZlibDecoder::new(input).read_to_end(&mut serialized).unwrap();

    // deserialize the uncompressed data
    let mut deserialized : compressed_image = protobuf::parse_from_bytes(serialized.as_slice()).unwrap();

    let width = deserialized.get_width();
    let height = deserialized.get_height();

    //perform the decompression
    let uncompressed_red = uncompress_color_channel(width, height, deserialized.take_red());
    let uncompressed_green = uncompress_color_channel(width, height, deserialized.take_green());
    let uncompressed_blue = uncompress_color_channel(width, height, deserialized.take_blue());
    let uncompressed_alpha = uncompress_color_channel(width, height, deserialized.take_alpha());

    //create the output image and load the data in
    let mut output_image = image::ImageBuffer::new(width, height);

    for x in 0..width {
        for y in 0..height {
            let pixel = image::Rgba([
                convert_to_8bit(uncompressed_red[(x + y * width) as usize]),
                convert_to_8bit(uncompressed_green[(x + y * width) as usize]),
                convert_to_8bit(uncompressed_blue[(x + y * width) as usize]),
                convert_to_8bit(uncompressed_alpha[(x + y * width) as usize]),
                ]);

            output_image.put_pixel(x,y,pixel);
        }
    }

    let _ = image::ImageRgba8(output_image).save(output, image::PNG);
}

fn uncompress_color_channel(width: u32, height: u32, compressed_channel_data: Vec<i32>) -> Vec<f32> {
    let mut result: Vec<f32> = Vec::with_capacity(width as usize * height as usize);

    let mut dct = dct::DCT3::new(width as usize);

    //the compressed data is encoded as a difference between rows. to decode it, we'll add each row to sum_row and then decode sum_row
    let mut sum_row = vec![0_i32; width as usize];
    let mut dct_output = vec![0_f32; width as usize];

    for (row_index, row_data) in compressed_channel_data.chunks(width as usize).enumerate() {
        for(sum_entry, encoded_entry) in sum_row.iter_mut().zip(row_data.iter()) {
            *sum_entry += *encoded_entry;
        }

        let decoded_row = quantize::decode(sum_row.as_slice());
        dct.process(decoded_row.as_slice(), dct_output.as_mut_slice());

        //we need to scale each element by 2/N in order to get back to the original data
        for element in dct_output.iter_mut() {
            *element *= 2_f32 / (width as f32);
        }

        result.extend(&dct_output);

        if row_index%200 == 0 {
            println!("{}", row_index);
        }
    }

    return result;
}

fn convert_to_8bit(item: f32) -> u8 {
    let result = item + 128_f32;

    if result > 255_f32 {
        return 255;
    }
    else if result < 0_f32 {
        return 0;
    }
    else {
        return result.round() as u8;
    }
}
