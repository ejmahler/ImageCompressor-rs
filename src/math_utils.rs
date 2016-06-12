
use std::f32;

pub fn dct_type_1(input: &[f32]) -> Vec<f32> {

    let size = input.len();
    let mut result = Vec::with_capacity(size);

    for k in 0..size {
        let mut current_value = 0_f32;

        let k_float = k as f32;

        for i in 1..(size - 1) {
            let i_float = i as f32;
            current_value += input[i] * (f32::consts::PI * k_float * i_float / ((size - 1) as f32)).cos();
        }

        current_value *= 2_f32;

        current_value += input[0];
        current_value += input[size - 1] *
                         (if k % 2 == 0 {
            1_f32
        } else {
            -1_f32
        });
        result.push(current_value * 0.5_f32);

    }

    return result;
}

pub fn dct_type_2(input: &[f32]) -> Vec<f32> {

    let mut result = Vec::with_capacity(input.len());
    let size_float = input.len() as f32;

    for k in 0..input.len() {
        let mut current_value = 0_f32;

        let k_float = k as f32;

        for i in 0..(input.len()) {
            let i_float = i as f32;

            current_value += input[i] * (f32::consts::PI * k_float * (i_float + 0.5_f32) / size_float).cos();
        }
        result.push(current_value);

    }

    return result;
}

pub fn dct_type_3(input: &[f32]) -> Vec<f32> {
    let mut result = Vec::with_capacity(input.len());

    let size_float = input.len() as f32;

    for k in 0..input.len() {
        let mut current_value = input[0] * 0.5_f32;

        let k_float = k as f32;

        for i in 1..(input.len()) {
            let i_float = i as f32;

            current_value += input[i] * (f32::consts::PI * i_float * (k_float + 0.5_f32) / size_float).cos();
        }
        result.push(current_value);

    }

    return result;
}

#[cfg(test)]
mod test {
    use super::*;

    fn compare_float_vectors(expected: &Vec<f32>, observed: &Vec<f32>) {
        assert_eq!(expected.len(), observed.len());

        for i in 0..expected.len() {
            if (observed[i] - expected[i]).abs() > 0.00001_f32 {
                println!("{:?}", observed);
                println!("{:?}", expected);
            }
            assert!((observed[i] - expected[i]).abs() < 0.00001_f32);
        }
    }

    fn scale_float_vec(input: &Vec<f32>, scale: f32) -> Vec<f32> {
        let mut result = Vec::with_capacity(input.len());
        for i in 0..input.len() {
            result.push(input[i] * scale);
        }
        return result;
    }

    #[test]
    fn test_dct_1_zeros() {
        let input = vec![0_f32, 0_f32, 0_f32, 0_f32, 0_f32];
        let output = dct_type_1(&input);
        let expected = vec![0_f32, 0_f32, 0_f32, 0_f32, 0_f32];

        compare_float_vectors(&expected, &output);

        let inverse = dct_type_1(&output);
        let scale = 2_f32 / (input.len() - 1) as f32;
        compare_float_vectors(&input, &scale_float_vec(&inverse, scale));
    }

    #[test]
    fn test_dct_1_ones() {
        let input = vec![1_f32, 1_f32, 1_f32, 1_f32, 1_f32];
        let output = dct_type_1(&input);
        let expected = vec![4_f32, 0_f32, 0_f32, 0_f32, 0_f32];

        compare_float_vectors(&expected, &output);

        let inverse = dct_type_1(&output);
        let scale = 2_f32 / (input.len() - 1) as f32;
        compare_float_vectors(&input, &scale_float_vec(&inverse, scale));
    }

    #[test]
    fn test_dct_1_arbitrary() {
        let input = vec![4_f32, 1_f32, 6_f32, 2_f32, 8_f32];
        let output = dct_type_1(&input);
        let expected = vec![15_f32, -2.707107_f32, 0_f32, -1.2928932_f32, 9_f32];

        compare_float_vectors(&expected, &output);

        let inverse = dct_type_1(&output);
        let scale = 2_f32 / (input.len() - 1) as f32;
        compare_float_vectors(&input, &scale_float_vec(&inverse, scale));
    }







    #[test]
    fn test_dct_2_and_3_zeros() {
        let input = vec![0_f32, 0_f32, 0_f32, 0_f32, 0_f32];
        let output = dct_type_2(&input);
        let expected = vec![0_f32, 0_f32, 0_f32, 0_f32, 0_f32];

        compare_float_vectors(&expected, &output);

        let inverse = dct_type_3(&output);
        let scale = 2_f32 / input.len() as f32;
        compare_float_vectors(&input, &scale_float_vec(&inverse, scale));
    }

    #[test]
    fn test_dct_2_and_3_ones() {
        let input = vec![1_f32, 1_f32, 1_f32, 1_f32, 1_f32];
        let output = dct_type_2(&input);
        let expected = vec![5_f32, 0_f32, 0_f32, 0_f32, 0_f32];

        compare_float_vectors(&expected, &output);

        let inverse = dct_type_3(&output);
        let scale = 2_f32 / input.len() as f32;
        compare_float_vectors(&input, &scale_float_vec(&inverse, scale));
    }

    #[test]
    fn test_dct_2_and_3_arbitrary() {
        let input = vec![4_f32, 1_f32, 6_f32, 2_f32, 8_f32];
        let output = dct_type_2(&input);
        let expected =
            vec![21_f32, -4.39201132_f32, 2.78115295_f32, -1.40008449_f32, 7.28115295_f32];

        compare_float_vectors(&expected, &output);

        let inverse = dct_type_3(&output);
        let scale = 2_f32 / input.len() as f32;
        compare_float_vectors(&input, &scale_float_vec(&inverse, scale));
    }
}
