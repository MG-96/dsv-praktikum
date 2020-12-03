use my_complex::Complex;

fn main() {
    let rot = Complex{ re: 0.995004165278, im: 0.099833416646};
    let mut sin = Complex{ re: 1.0, im: 0.0 };

    for _ in 0..500 {
        println!("{}", sin.re);

        sin = sin*rot;
    }
}

