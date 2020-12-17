mod convolve;

fn main() {
    let abc = convolve::convolve(&vec![1,1,1,1,1], &vec![1,1,1,1,1]);
    println!("{:?}", abc)

}
