#[derive(Clone)]  // <-- wird glaub ich hier nicht zwingend benötigt
pub struct UnitDelay<T: std::clone::Clone> {
    prev: T
}

impl<T: std::clone::Clone> UnitDelay<T> {
    pub fn new(start: T) -> UnitDelay<T> {
        UnitDelay { prev: start}
    }
    pub fn process(&mut self, x: T) -> T {
        let tmp = self.prev.clone();
        self.prev = x;
        return tmp
    }
    pub fn process_in_place(&mut self, x: &mut T) {
        let tmp = x.clone();
        *x = self.prev.clone();
        self.prev = tmp;
    }
}
impl<T: std::clone::Clone> UnitDelay<Vec<T>> {
    pub fn process_batch(&mut self, vector: Vec<T>) -> Vec<T> {
        let mut n = vector.len();
        if n > self.prev.len() {
            n = self.prev.len();
        }
        self.prev.append(&mut vector.clone());
        return self.prev.drain(0..n).collect()
    }
}

mod filter {
    pub struct SimpleFilter<T: Clone + std::ops::Add + std::ops::Add<Output = T> + Copy> {
        delay: super::UnitDelay<T>,
    }

    impl<T: Clone + std::ops::Add + std::ops::Add<Output = T> + Copy> SimpleFilter<T> {
        pub fn process(&mut self, n: T) -> T {
            return n + self.delay.process(n)
        }
    }
}


#[cfg(test)]
mod tests {
    #[test]
    fn simple_process() {
        let mut delay = super::UnitDelay{prev: 0 as i16};
        let out = delay.process(5);
        assert_eq!(out, 0);
        let out = delay.process(6);
        assert_eq!(out, 5);
    }

    #[test]
    fn vec_test() {
        // here we test if batch delaying works
        // its important to return n elements when proccess is called with n elements
        let mut delay = super::UnitDelay{prev: vec![1 as i16, 2, 3]};
        let out = delay.process_batch(vec![4, 5, 6, 7, 8]);
        assert_eq!(out, vec![1,2,3]);
        assert_eq!(delay.prev, vec![4,5,6,7,8]);
    }

    #[test]
    fn name() {
        let mut delay = super::UnitDelay{prev: 0 as i16};
        let mut inout: i16 = 5;
        delay.process_in_place(&mut inout);
        assert_eq!(inout, 0);
        delay.process_in_place(&mut inout);
        assert_eq!(inout, 5);
    }
}
