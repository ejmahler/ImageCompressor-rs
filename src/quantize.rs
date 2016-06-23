const NUM_ZONES: usize = 2000;

pub fn encode(input: &[f32]) -> Vec<i32> {
    let quantize_vec = make_quantize_vec(input.len());

    let mut result = Vec::with_capacity(input.len());

    for (x, q) in input.iter().zip(quantize_vec.iter()) {
        result.push((x / q).round() as i32);
    }

    return result;
}

pub fn decode(input: &[i32]) -> Vec<f32> {
    let quantize_vec = make_quantize_vec(input.len());

    let mut result = Vec::with_capacity(input.len());

    for (x, q) in input.iter().zip(quantize_vec.iter()) {
        result.push((*x as f32) * q);
    }

    return result;
}

fn make_quantize_vec(size: usize) -> Vec<f32> {

    let mut result = Vec::with_capacity(size);

    result.push(1_f32);

    for i in 1..size {
        result.push((i * 2 + NUM_ZONES) as f32);
    }

    return result;
}
