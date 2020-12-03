use std::ops;

#[derive(Debug,Copy,Clone)]
pub struct Complex {
    pub re: f64,
    pub im: f64
}

// Methods of Complex Type
impl Complex {
    pub fn conj(&mut self) {
        self.im = -self.im;
    }
    pub fn arg(&self) -> f64{
        use std::f64::consts::PI;
        // first test for special cases
        if self.re == 0.0 {
            if self.im > 0.0 {
                return PI/2.0
            } else if self.im < 0.0 {
                return -PI/2.0
            } else {
                return 0.0
            }
        }
        return self.im.atan2(self.re)
    }
    pub fn set_arg(&mut self, arg: f64) {
        self.re = self.mag() * arg.cos();
        self.im = self.mag() * arg.sin();
    }
    pub fn mag(&self) -> f64{
        (self.re*self.re+self.im*self.im).sqrt()
    }
    pub fn set_mag(&mut self, mag: f64) {
        self.re = mag * self.arg().cos();
        self.im = mag * self.arg().sin();
    }
}
// Operator overloding of Complex Type
impl ops::Add<Complex> for Complex {
    type Output = Complex;

    fn add(self, other: Complex) -> Complex {
        Complex {re: self.re+other.re, im: self.im+other.im }
    }
}
impl ops::Sub<Complex> for Complex {
    type Output = Complex;

    fn sub(self, other: Complex) -> Complex {
        Complex {re: self.re-other.re, im: self.im-other.im }
    }
}
impl ops::Mul<Complex> for Complex {
    type Output = Complex;

    fn mul(self, other: Complex) -> Complex {
        let re = self.re*other.re-self.im*other.im;
        let im = self.re*other.im+self.im*other.re;
        Complex {re, im}
    }
}

#[cfg(test)]
mod tests {
    use crate::Complex;
    #[test]
    fn simple_conj() {
        let mut z = Complex {re : 0.0, im: 5.0};
        z.conj();
        assert_eq!(z.im, -5.0);
        z.conj();
        assert_eq!(z.im, 5.0);
    }
    #[test]
    fn comp_add() {
        let a = Complex {re: 2.0, im: -2.0};
        let b = Complex {re: 2.0, im: -2.0};
        
        let z = a+b;
        assert_eq!(z.re, 4.0);
        assert_eq!(z.im, -4.0);
    }
    #[test]
    fn comp_sub() {
        let a = Complex {re: 4.0, im: -4.0};
        let b = Complex {re: 2.0, im: -2.0};
        
        let z = a-b;
        assert_eq!(z.re, 2.0);
        assert_eq!(z.im, -2.0);
    }
    #[test]
    fn comp_mult() {
        let a = Complex {re: 4.0, im: -4.0};
        let b = Complex {re: 2.0, im: -2.0};
        
        let z = a*b;
        assert_eq!(z.re, 0.0);
        assert_eq!(z.im, -16.0);
    }
    #[test]
    fn get_mag() {
        // tests for getting magnitude of complex numbers
        let a = Complex {re: 2.0, im: 2.0};
        
        assert_eq!(a.mag(), 8.0_f64.sqrt());
    }
    #[test]
    fn get_arg() {
        // tests for getting arguments part of complex numbers
        let mut z = Complex {re:2.0, im: 2.0};

        assert_eq!(z.arg(), std::f64::consts::PI/4.0);

        // now test some special cases

        z.re = 0.0;
        assert_eq!(z.arg(), std::f64::consts::PI/2.0);

        z.conj();
        assert_eq!(z.arg(), -std::f64::consts::PI/2.0);

        z.im = 0.0;
        assert_eq!(z.arg(), 0.0);
    }
}

