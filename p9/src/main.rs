struct Delay<T: std::default::Default> {
    prev: Vec<T>
}

impl<T: std::default::Default + Clone> Delay<T> {
    pub fn delay(&mut self, data: &mut Vec<T>, n: usize) {
        if self.prev.len() < n {
            let z = n - self.prev.len();
            let mut zeros = vec![T::default(); z];
            zeros.append(&mut self.prev);
            self.prev = zeros;
        }
        self.prev.append(data);
        *data = self.prev.drain(0..self.prev.len()-n).collect();
    }
}

use wav;


fn amplify(data: &mut Vec<i16>, factor: f32) {
    for v in data {
        *v = (*v as f32 * factor) as i16;
    }
}

fn echo_1x(data: &mut Vec<i16>, delay: usize, damping: f32) {
    let mut d: Delay<i16> = Delay{prev: Vec::new()};

    let mut delayed = data.clone();
    d.delay(&mut delayed, delay);
    amplify(&mut delayed, damping);

    for n in 0..data.len() {
        data[n] = data[n]+delayed[n];
    }
}

fn echo_nx(data: &mut Vec<i16>, delay: usize, damping: f32) {
    let mut d: Delay<i16> = Delay{prev: Vec::new()};

    let mut nd = damping;
    let mut delayed = data.clone();
    let mut n = 2;
    while nd>f32::EPSILON {
        d.prev = Vec::new();
        println!("Damping: {:?}", nd);
        d.delay(&mut delayed, delay);
        amplify(&mut delayed, nd);

        for n in 0..data.len() {
            data[n] = data[n]+delayed[n];
        }

        nd = damping.powi(n);
        n +=1;
    }

}


fn main() {
    use std::fs::File;
    use std::path::Path;

    let mut inp_file = File::open(Path::new("data/dsv-praktikum.wav")).unwrap();
    let (header, mut data) = wav::read(&mut inp_file).unwrap();

    if let wav::BitDepth::Sixteen(ref mut d) = data {
        echo_1x(d, (0.5*48_000.0) as usize, 0.8);
    }

    let mut out_file = File::create(Path::new("data/echo1x_dsv-praktikum.wav")).unwrap();
    wav::write(header, &data, &mut out_file).unwrap();


    if let wav::BitDepth::Sixteen(ref mut d) = data {
        echo_nx(d, (0.5*48_000.0) as usize, 0.6);
    }

    let mut out_file = File::create(Path::new("data/echonx_dsv-praktikum.wav")).unwrap();
    wav::write(header, &data, &mut out_file).unwrap();

}

#[cfg(test)]
mod tests {
    #[test]
    fn test_delay() {
    let mut data = vec![1 as i16,2,3,4,5];
    let mut delay: super::Delay<i16> = super::Delay{prev: Vec::new()};

    delay.delay(&mut data, 2);
    assert_eq!(data, &[0,0,1,2,3]);

    let mut data = vec![6 as i16,7,8,9,10];
    delay.delay(&mut data, 2);
    assert_eq!(data, &[4,5,6,7,8]);

    }
}