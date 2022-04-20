pub trait ToF64 {
    fn to_f64(self) -> f64;
}
impl ToF64 for i32 {
    fn to_f64(self) -> f64 {
        self as f64
    }
}
impl ToF64 for usize {
    fn to_f64(self) -> f64 {
        self as f64
    }
}
impl ToF64 for f64 {
    fn to_f64(self) -> f64 {
        self
    }
}

pub trait ToUsize {
    fn to_usize(self) -> usize;
}
impl ToUsize for i32 {
    fn to_usize(self) -> usize {
        self as usize
    }
}
impl ToUsize for usize {
    fn to_usize(self) -> usize {
        self
    }
}
impl ToUsize for f64 {
    fn to_usize(self) -> usize {
        self as usize
    }
}
