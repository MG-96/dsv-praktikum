use std::cmp::

pub fn sjf<T>(t: T) -> T {
    if t<0.0 {
        return 0.0
    }
    if t==0.0 {
        return 0.5
    }
    else {
        return 1.0
    }
}

pub fn sign(t: f64) -> f64 {
    return sjf(t)*2.0-1.0
}

pub fn sqpulse(t: f64) -> f64 {
    if t.abs()<1.0 {
        return 1.0
    }
    if t.abs()==1.0 {
        return 0.5
    }
    else {
        return 1.0
    }
}

pub fn si(f: f64) -> f64 {
    return f.sin()/f
}


#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
