use std::f32;
use rustfft;
use num::{Complex, Zero, FromPrimitive, Signed};

pub struct DCT2<T> {
    fft: rustfft::FFT<T>,
    fft_input: Vec<Complex<T>>,
    fft_output: Vec<Complex<T>>,

    output_correction: Vec<Complex<T>>,
}

impl<T> DCT2<T> where T: Signed + FromPrimitive + Copy {
    /// Creates a new DCT2 context that will process signals of length `len`.
    pub fn new(len: usize) -> Self {
        let fft = rustfft::FFT::new(len, false);
        DCT2 {
            fft: fft,
            fft_input: vec![Complex::new(Zero::zero(),Zero::zero()); len],
            fft_output: vec![Complex::new(Zero::zero(),Zero::zero()); len],
            output_correction: (0..len)
                .map(|i| i as f32 * 0.5 * f32::consts::PI / len as f32)
                .map(|phase| Complex::from_polar(&1.0, &phase).conj())
                .map(|c| Complex {re: FromPrimitive::from_f32(c.re).unwrap(),
                                im: FromPrimitive::from_f32(c.im).unwrap()})
                .collect(),
        }
    }

    /// Runs the DCT2 on the input `signal` buffer, and places the output in the
    /// `spectrum` buffer.
    ///
    /// # Panics
    /// This method will panic if `signal` and `spectrum` are not the length
    /// specified in the struct's constructor.
    pub fn process(&mut self, signal: &[T], spectrum: &mut [T]) {

        //we're going to convert this to a FFT. we'll do so by redordering the inputs, running the FFT, and then multiplying by a correction factor
        assert!(signal.len() == self.fft_input.len());

        //the first half of the array will be the even elements, in order
        let even_end = (signal.len()+1)/2;
        for i in 0..even_end {
            unsafe {
                *self.fft_input.get_unchecked_mut(i) = Complex::from(*signal.get_unchecked(i*2));
            }
        }

        //the second half is the odd elements in reverse order
        let odd_end = signal.len() - 1 - signal.len()%2;
        for i in 0..signal.len()/2 {
            unsafe {
                *self.fft_input.get_unchecked_mut(even_end + i) = Complex::from(*signal.get_unchecked(odd_end - 2*i));
            }
        }

        //run the fft
        self.fft.process(&self.fft_input, &mut self.fft_output);

        //apply a correction factor to the result
        for i in 0..signal.len() {
            unsafe {
                *spectrum.get_unchecked_mut(i) = (*self.fft_output.get_unchecked(i) * *self.output_correction.get_unchecked(i)).re;
            }
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use std::f32;

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


    fn execute_slow(input: &[f32]) -> Vec<f32> {

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


    #[test]
    fn test_slow() {
        let input = vec![4_f32, 1_f32, 6_f32, 2_f32, 8_f32];
        let output = execute_slow(&input);
        let expected =
            vec![21_f32, -4.39201132_f32, 2.78115295_f32, -1.40008449_f32, 7.28115295_f32];

        compare_float_vectors(&expected.as_slice(), &output.as_slice());
    }

    #[test]
    fn test_fast() {
        let input = vec![4_f32, 1_f32, 6_f32, 2_f32, 8_f32];
        let slow_output = execute_slow(&input);

        let mut dct = DCT2::new(input.len());
        let mut fast_output = input.clone();
        dct.process(&input, &mut fast_output);

        println!("{:?}", slow_output);
        println!("{:?}", fast_output);

        compare_float_vectors(&slow_output.as_slice(), &fast_output);
    }
}