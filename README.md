# ImageCompressor-rs

This is a novelty image compression program. It can compress PNG files to the custom "mcsa" format, and uncompress them back into PNG files.

Its algorithm is similar to jpeg's algorithm, with the same 2d DCT and quantization steps -- but while jpeg operates on 8x8 blocks, this algorithm operates on an entire color channel at once.

# Building
This project requires Rust 1.9 or later, but no other dependencies. Build with `cargo build --release`

# Usage
## Compression
./image_compressor compress myImage.png [--output=target.msca]

If the output path isn't specified, it'll be the input path, with the extension changed to ".msca". There are no other supported arguments at this time.

## Uncompression
./image_compressor uncompress myImage.msca [--output=target.png]

If the output path isn't specified, it'll be the input path, with the extension changed to ".png". There are no other supported arguments at this time.


# Performance
The primary factor in performance is in the divisibility of the uimage size. The compression involves a 2-dimensional FFT, and the underlying algorithm performs a prime factorization on the image dimensions. Images with many small prime factors will compute very quickly, while images with prime width or height will compute very slowly.

# License
GPL v3