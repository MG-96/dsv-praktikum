mod sin_gen {
    use std::f32::consts::PI;
    pub struct Sinus {
        pub a: f32,
        pub freq: f32,
        pub phase: f32,
        s_rate: u32,
        n: f32
    }
    impl Sinus {
        pub fn new(a: f32, freq: f32, phase: f32, s_rate: u32) -> Sinus {
            Sinus {a : a, freq: freq, phase: phase, s_rate: s_rate, n: 0.0}
        }
        pub fn process(&mut self) -> i16 {
            let sin = (self.a * (2.0*PI*self.freq* self.n + self.phase).sin() * i16::MAX as f32) as i16;
            self.n += (1.0/self.s_rate as f32) % (2.0*PI);
            return sin;
        }
        pub fn get(&mut self) -> i16 {
            (self.a * (2.0*PI*self.freq* self.n + self.phase).sin() * i16::MAX as f32) as i16
        }
        pub fn next(&mut self) {
            self.n += 1.0/self.s_rate as f32;
        }
    }
}

use std::fs::File;
use std::path::Path;
fn main() {
    sin_sweep();
    rect();
}

fn rect() {
    let mut out_file = File::create(Path::new("data/rect.wav")).expect("failed to open file");

    let header = wav::Header::new(1, 1, 48_000, 16);
    let mut data: Vec<i16> = Vec::new();

    let mut gen = sin_gen::Sinus::new(1.0, 200.0, 0.0, 48_000);
    for j in 1..20 {
        for _i in 0..48_000*2 {
            let mut value: i16 = 0;
            for s in 1..j*2 {
                if s%2 != 0 {
                    gen.freq = (100*s) as f32;
                    gen.a = 1.0/s as f32;
                    value += gen.get();
                }
            }
            gen.next();
            data.push(value);
        }
    }

    wav::write(header, wav::BitDepth::Sixteen(data), &mut out_file).expect("failed to write wav");
}

fn sin_sweep() {

    let mut out_file = File::create(Path::new("data/sin_sweep.wav")).expect("failed to open file");

    let header = wav::Header::new(1, 1, 48_000, 16);
    let mut data: Vec<i16> = Vec::new();

    let mut gen = sin_gen::Sinus::new(1.0, 200.0, 0.0, 48_000);
    for i in 0..48_000*2 {
        gen.freq = i as f32 *0.02;
        data.push(gen.process());
    }

    wav::write(header, wav::BitDepth::Sixteen(data), &mut out_file).expect("failed to write wav");
}
