use num::complex::*;
use std::f64::consts::PI;

pub fn dft(data: &Vec<f64>) -> Vec<Complex64> {
    let omega = (2.0*PI)/data.len() as f64;
    let mut fs: Vec<Complex64> = Vec::new();
    for mu in 0..data.len() {
        let mut xm = Complex64::default();
        for k in 0..data.len() {
            xm += Complex::new(data[k], f64::default()) * Complex::from_polar(1.0 , -omega*k as f64*mu as f64);
        }
        fs.push(xm);
    }
    fs
}

pub fn idft(data: &Vec<Complex64>) -> Vec<f64> {
    let omega = (2.0*PI)/data.len() as f64;
    let mut fs: Vec<f64> = Vec::new();
    for mu in 0..data.len() {
        let mut xm = Complex64::default();
        for k in 0..data.len() {
            xm += data[k] * Complex::from_polar(1.0 , omega*k as f64*mu as f64);
        }
        fs.push(xm.re/data.len() as f64);
    }
    fs
}