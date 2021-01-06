use wav::BitDepth;

fn reduce_bit_depth(data: &mut BitDepth, targed_depth: usize) {

    match data {
        BitDepth::Eight(s) => {
            for v in s {
                *v = *v>>(8-targed_depth);
                *v = *v<<(8-targed_depth);
            }
        }
        BitDepth::Sixteen(s) => {
            for v in s {
                *v = *v>>(16-targed_depth);
                *v = *v<<(16-targed_depth);
            }
        }
        BitDepth::TwentyFour(s) => {
            for v in s {
                *v = *v>>(24-targed_depth);
                *v = *v<<(24-targed_depth);
            }
        }
        _ => {}
    }
}

fn amplify(data: &mut BitDepth, factor: f32) {
    match data {
        BitDepth::Eight(s) => {
            for v in s {
                *v = (*v as f32 * factor) as u8;
            }
        }
        BitDepth::Sixteen(s) => {
            for v in s {
                *v = (*v as f32 * factor) as i16;
            }
        }
        _ => {}
    }

}

fn amplify_with_wrap(data: &mut BitDepth, factor: i16) {
    match data {
        BitDepth::Eight(s) => {
            for v in s {
                *v = *v * factor as u8;
            }
        }
        BitDepth::Sixteen(s) => {
            for v in s {
                *v = *v * factor;
            }
        }
        _ => {}
    }

}

fn main() {
    use std::fs::File;
    use std::path::Path;


    let mut inp_file = File::open(Path::new("data/dsv-praktikum.wav")).unwrap();
    let (header, mut data) = wav::read(&mut inp_file).unwrap();

    // reduce_bit_depth(&mut data, 4);

    // let mut out_file = File::create(Path::new("data/4bit-dsv-praktikum.wav")).unwrap();
    // wav::write(header, &data, &mut out_file).unwrap();

    // amplify(&mut data, 60.0);
    // let mut out_file = File::create(Path::new("data/amp-dsv-praktikum.wav")).unwrap();
    // wav::write(header, &data, &mut out_file).unwrap();

    amplify_with_wrap(&mut data, 10);
    let mut out_file = File::create(Path::new("data/amp-with-wrap-dsv-praktikum.wav")).unwrap();
    wav::write(header, &data, &mut out_file).unwrap();
}

#[cfg(test)]
mod tests {
    #[test]
    fn positive_numbers_test() {
        let mut test = wav::BitDepth::Eight(vec![0b01111001]);
        super::reduce_bit_depth(&mut test, 4);
        if let wav::BitDepth::Eight(s) = test {
            assert_eq!(s[0], 0b01110000);
        }
        let mut test = wav::BitDepth::Sixteen(vec![0b0111_1111_1111_1111]);
        super::reduce_bit_depth(&mut test, 8);
        if let wav::BitDepth::Sixteen(s) = test {
            assert_eq!(s[0], 0b0111_1111_0000_0000);
        }
    }

    #[test]
    fn negative_numbers_test() {
        let mut test = wav::BitDepth::Sixteen(vec![-30000]);
        super::reduce_bit_depth(&mut test, 8);
        if let wav::BitDepth::Sixteen(s) = test {
            assert_eq!(s[0], -30208);
        }
    }
}