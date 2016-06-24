
pub fn encode(width: usize, height: usize, input: &[f32]) -> Vec<i32> {
    let mut result = Vec::with_capacity(input.len());

    for (y, row_data) in input.chunks(width).enumerate() {
        for (x, item) in row_data.iter().enumerate() {
            let coeff = get_quantize_coeff(width, height, x, y);
            result.push((item / coeff).round() as i32);
        }
    }

    return result;
}

pub fn decode(width: usize, height: usize, input: &[i32]) -> Vec<f32> {
    let mut result = Vec::with_capacity(input.len());

    for (y, row_data) in input.chunks(width).enumerate() {
        for (x, item) in row_data.iter().enumerate() {
            let coeff = get_quantize_coeff(width, height, x, y);
            result.push((*item as f32) * coeff);
        }
    }

    return result;
}

fn get_quantize_coeff(width: usize, height: usize, x: usize, y: usize) -> f32 {
    return ((width + height) / 10 + (x + y) / 10) as f32;
}
