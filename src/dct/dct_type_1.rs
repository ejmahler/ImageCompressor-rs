

#[cfg(test)]
mod test {
    use std::f32;
    use super::*;


    fn fuzzy_cmp(a: f32, b: f32, tolerance: f32) -> bool {
        a >= b - tolerance && a <= b + tolerance
    }

    fn compare_float_vectors(expected: &[f32], observed: &[f32]) {
        assert_eq!(expected.len(), observed.len());

        let tolerance : f32 = 0.0001;

        for i in 0..expected.len() {
            assert!(fuzzy_cmp(observed[i], expected[i], tolerance));
        }
    }


    pub fn execute_slow(input: &[f32]) -> Vec<f32> {

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


    #[test]
    fn test_slow() {
        let input = vec![4_f32, 1_f32, 6_f32, 2_f32, 8_f32];
        let output = execute_slow(&input);
        let expected = vec![15_f32, -2.707107_f32, 0_f32, -1.2928932_f32, 9_f32];

        println!("{:?}", output);

        compare_float_vectors(&expected.as_slice(), &output.as_slice());
    }
}
