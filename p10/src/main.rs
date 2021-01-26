mod dfts;

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
    }
}

fn sinus_tests() {
    let mut sin = sin_gen::Sinus::new(1.0, 100.0, 0.0, 10000);
    let mut x1: Vec<f64> = Vec::new();

    for _n in 0..1000 {
        x1.push(sin.process() as f64);
    }
    // println!("{:?}", x1);

    let fx1 = dfts::dft(&x1);
    let ifx1 = dfts::idft(&fx1);

    for x in ifx1 {
        println!("{:?}", x);
    }
}

fn faltung() {
    let mut rect: Vec<f64> = Vec::new();
    
    // for _i in 0..500 {
    //     rect.push(0.0);
    // }
    for _i in 0..100 {
        rect.push(1.0);
    }
    for _i in 0..100 {
        rect.push(0.0);
    }

    let mut file1 = File::create(Path::new("data/frect.csv")).unwrap();

    let frect = dfts::dft(&rect);
    for x in &frect {
        file1.write_all(format!("{}\n", x.to_polar().0).as_bytes()).unwrap();
    }

    let mfrect: Vec<Complex64> = frect.iter().map(|x| x*x).collect();

    let folded = dfts::idft(&mfrect);
    for x in folded {
        println!("{:?}", x);
    }

}

use num::complex::*;
use std::path::Path;
use std::fs::File;
use std::io::prelude::*;
use wav::*;

fn sa() {
    let mut inp_file = File::open(Path::new("data/dsv-praktikum.wav")).unwrap();
    let mut out_file = File::create(Path::new("data/dsv-praktikum_spectrum.csv")).unwrap();
    let (_header, data) = wav::read(&mut inp_file).unwrap();

    let mut dvector: Vec<f64> = Vec::new();

    if let BitDepth::Sixteen(v) = data {
        for x in v {
            dvector.push(x as f64);
        }
    }
    println!("Processing {} elements...", dvector.len());
    println!("Resulting in {} iterations...", dvector.len()*dvector.len());

    let output = dfts::dft(&dvector);
    for x in &output {
        out_file.write_all(format!("{}\n", x.to_polar().0).as_bytes()).unwrap();
    }
}
    

fn main() {
    sa();
}
