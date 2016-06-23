extern crate argparse;
extern crate image;
extern crate protobuf;
extern crate flate2;
extern crate num;
extern crate rustfft;

use argparse::{ArgumentParser, Store};

use std::str::FromStr;
use std::path::Path;

mod compress;
mod uncompress;
mod quantize;
mod compressed_image;
mod dct;

#[allow(non_camel_case_types)]
enum Command {
    compress,
    uncompress,
}

impl FromStr for Command {
    type Err = ();
    fn from_str(src: &str) -> Result<Command, ()> {
        return match src {
            "compress" => Ok(Command::compress),
            "uncompress" => Ok(Command::uncompress),
            _ => Err(()),
        };
    }
}

fn main() {

    let mut input_filename = "".to_string();
    let mut output_filename = "".to_string();

    let mut command = Command::compress;

    {
        let mut parser = ArgumentParser::new();
        parser.set_description("Compress or uncompress an image");
        parser.refer(&mut command)
            .required()
            .add_argument("command",
                          Store,
                          r#"Command to run (either "compress" or "uncompress")"#);

        parser.refer(&mut input_filename)
            .required()
            .add_argument("filename", Store, "Input filename");
        parser.refer(&mut output_filename)
            .add_option(&["-o", "--output"], Store, "Output file");
        parser.parse_args_or_exit();
    }

    match command {
        Command::compress => {
            // if the user supplied the optional output filename, supply it directly
            // otherwise call a different method to autogenerate it
            if output_filename.len() > 0 {
                compress::compress_file_to_output(Path::new(&input_filename),
                                                  Path::new(&output_filename))
            } else {
                compress::compress_file(Path::new(&input_filename))
            }
        }
        Command::uncompress => {
            // if the user supplied the optional output filename, supply it directly
            // otherwise call a different method to autogenerate it
            if output_filename.len() > 0 {
                uncompress::uncompress_file_to_output(Path::new(&input_filename),
                                                      Path::new(&output_filename))
            } else {
                uncompress::uncompress_file(Path::new(&input_filename))
            }
        }
    }
}
