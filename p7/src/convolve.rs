pub fn convolve<T: std::ops::Mul<Output = T> + std::ops::Add<Output = T> + Copy + Default>(s1: &Vec<T>, s2: &Vec<T>) -> Vec<T>{
    let mut out: Vec<T> = Vec::new();

    for n in 0..s1.len()+s2.len()-1 {
        let mut xh = T::default();
        for k in 0..s1.len() {
            if k > n {
                break;
            }
            if n-k >= s2.len() {
                continue;
            }
            xh = xh + s1[k]*s2[n-k];
        }
        out.push(xh);
    }
    out
}