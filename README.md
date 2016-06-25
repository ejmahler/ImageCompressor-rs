# ImageCompressor-rs

This is a novelty image compression program. It can compress PNG files to the custom "mcsa" format, and uncompress them back into PNG files.

Its algorithm is similar to jpeg's algorithm, with the same 2d DCT and quantization steps -- but while jpeg operates on 8x8 blocks, this algorithm operates on an entire color channel at once.

It currently achieves comparable compression rates to jpeg at similar quality, although it's difficult to quantify the quality differences.

# Building
This project requires Rust 1.9 or later, but no other dependencies. Build with `cargo build --release`

# Usage
## Compression
`./image_compressor compress path/to/myImage.png [--output=otherPath/to/target.msca]`

If the output path isn't specified, it'll be the input path, with the extension changed to ".msca". There are no other supported arguments at this time.

## Uncompression
`./image_compressor uncompress path/to/myImage.msca [--output=otherPath/to/target.png]`

If the output path isn't specified, it'll be the input path, with the extension changed to ".png". There are no other supported arguments at this time.


# Performance
The compression algorithm involves a 2-dimensional FFT, and the underlying FFT algorithm performs a prime factorization on the image dimensions. Images with many small prime factors will compute very quickly, while images with prime width or height will compute very slowly.

# License
GPL v3
