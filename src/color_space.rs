
pub fn rgb_to_ycbcr(r: u8, g: u8, b: u8) -> (f32, f32, f32) {
	//using exact values from https://en.wikipedia.org/wiki/YCbCr#JPEG_conversion
	let rf = r as f32 - 128_f32;
	let gf = g as f32 - 128_f32;
	let bf = b as f32 - 128_f32;

	(
		0.299_f32 * rf + 0.587_f32 * gf + 0.114_f32 * bf, //luma channel
		128_f32 - 0.168736_f32 * rf - 0.331264_f32 * gf + 0.5_f32 * bf, //blue channel
		128_f32 + 0.5_f32 * rf - 0.418688 * gf - 0.081312 * bf
	)
}

pub fn ycbcr_to_rgb(y: f32, cb: f32, cr: f32) -> (u8, u8, u8) {
	//using exact values from https://en.wikipedia.org/wiki/YCbCr#JPEG_conversion

	let red = y + 1.402_f32 * (cr - 128_f32);
	let green = y - 0.34414_f32 * (cb - 128_f32) - 0.71414_f32 * (cr - 128_f32);
	let blue = y + 1.772_f32 * (cb - 128_f32);

	(
		clamp(0_f32, red + 128_f32, 255_f32).round() as u8,
		clamp(0_f32, green + 128_f32, 255_f32).round() as u8,
		clamp(0_f32, blue + 128_f32, 255_f32).round() as u8,
	)
}

fn clamp(min: f32, val: f32, max: f32) -> f32 {
	if val < min {
		min
	} else if val > max {
		max
	} else {
		val
	}
}

#[cfg(test)]
mod test {
	use super::*;

	#[test]
	fn test_inverse() {
		let color_list = vec![
			(0,0,0),
			(255,255,255),
			(12,75,34)
		];

		for (red, green, blue) in color_list {
			let (y, cb, cr) = rgb_to_ycbcr(red, green, blue);
			let (r,g,b) = ycbcr_to_rgb(y,cb,cr);

			assert_eq!(red, r);
			assert_eq!(green, g);
			assert_eq!(blue, b);
		}
	}

	#[test]
	fn test_colors() {
		{
			let (y, cb, cr) = rgb_to_ycbcr(0, 0, 0);

			assert_eq!(y, -128_f32);
			assert_eq!(cb, 128_f32);
			assert_eq!(cr, 128_f32);
		}

		{
			let (y, cb, cr) = rgb_to_ycbcr(255, 255, 255);

			assert_eq!(y,  127_f32);
			assert_eq!(cb, 128_f32);
			assert_eq!(cr, 128_f32);
		}
	}
}
