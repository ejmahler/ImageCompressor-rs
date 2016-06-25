
pub fn encode(width: usize, height: usize, input: &[f32]) -> Vec<i32> {

    //encode values in a zig zag ordr (like jpeg). omit the first set of contiguous zeroes.
    ZigZagIterator::new(width, height).map(|(x,y)| {
        let coeff = get_quantize_coeff(width, height, x, y);
        (input[x + y * width] / coeff).round() as i32
    }).skip_while(|item| { return *item == 0;}).collect()
}

pub fn decode(width: usize, height: usize, input: &[i32]) -> Vec<f32> {
    let mut result = vec![0_f32; width*height];

    //the first block of zeros is omitted. we'll use the difference between input.len() and width*height to tell how many
    let position_iter = ZigZagIterator::new(width, height).skip(width*height - input.len());

    //zip through the input and position data, putting unquantized values where they belong
    for (item, (x,y)) in input.iter().zip(position_iter) {
        let coeff = get_quantize_coeff(width, height, x, y);
        result[x + y*width] = (*item as f32) * coeff;
    }

    return result;
}

fn get_quantize_coeff(width: usize, height: usize, x: usize, y: usize) -> f32 {
    return ((width * height) / 10000 + (x + y) * 20) as f32;
}



//the zig zag iterator iterates through all x,y coordinates in a 2d array with size (width, height)
//it traverses through coornates in a diagonal, zig zag fashion (from lower left to upper right)
//it starts in the lower right, going back and forth across the whole thing until it hits (0,0)
struct ZigZagIterator {
    width: usize,
    height: usize,
    previous_result: (usize, usize),
}

impl ZigZagIterator {
    fn new(width: usize, height: usize) -> ZigZagIterator {

        //we want to set up our previous_result so that the first iteration isn't a special case
        //ie, put it right "before" the spot we want to start on
        ZigZagIterator {
            width: width,
            height: height,
            previous_result: if (width + height)%2 == 0 { (width - 1, height) } else { (width, height - 1) }
        }
    }
}

impl Iterator for ZigZagIterator {
    type Item = (usize, usize);

    fn next(&mut self) -> Option<(usize, usize)> {
        if self.previous_result == (0, 0) {
            None
        } else {
            //the diagonal index will determine whether we're going up/right or down/left
            let diagonal_index = self.previous_result.0 + self.previous_result.1;

            let next = if diagonal_index%2 == 1 {
                //if the index is odd, we're going up and right

                //first check if the y is already at 0. if it is, we can'tgo up any further
                if self.previous_result.1 == 0 {
                    (self.previous_result.0 - 1, 0)

                //next check if we've gone too far right. if we have, go straight up
                } else if self.previous_result.0 + 1 == self.width {
                    (self.width - 1, self.previous_result.1 - 1)

                //we're good to go up and right
                } else {
                    (self.previous_result.0 + 1, self.previous_result.1 - 1)
                }


            } else {
                //the diagonal index is even, so we're going down and left

                //first check if the x is already at 0. if it is, we can't go left any further, so go up instead
                if self.previous_result.0 == 0 {
                    (0, self.previous_result.1 - 1)

                //next check if we've gone too far down. if we have, go straight left
                } else if self.previous_result.1 + 1 == self.height {
                    (self.previous_result.0 - 1, self.height - 1)

                //we're good to go up and right
                } else {
                    (self.previous_result.0 - 1, self.previous_result.1 + 1)
                }
            };

            let result = Some(next);
            self.previous_result = next;
            result
        }
    }
}

#[cfg(test)]
mod test {
    use super::ZigZagIterator;

    #[test]
    fn test_zigzag() {

        //the 2 numbers in each tuple are the width and height, and the vec contains the expected order
        //the iterator will visit elements in (it's a flattened 2d array in row major order)
        let input_list = vec![
            (1,1,vec![0]),
            (1,5,vec![4,3,2,1,0]),
            (2,2,vec![3, 2,    1, 0]),
            (3,2,vec![5,4,1,   3,2,0]),
            (2,3,vec![5,4,   3,1,   2,0]),
            (3,3,vec![8,7,3,    6,4,2,    5,1,0]),
        ];

        for (width, height, expected) in input_list {
            println!("");

            let mut result = vec![0; width*height];
            for (seq, (x,y)) in ZigZagIterator::new(width, height).enumerate() {
                result[x + y * width] = seq;
            }

            println!("expected: {:?}", expected);
            println!("result:   {:?}", result);

            for (actual, expected) in result.iter().zip(expected.iter()) {
                assert_eq!( actual, expected );
            }
        }
    }
}
