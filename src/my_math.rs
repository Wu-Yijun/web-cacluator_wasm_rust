enum MyVar {
    MyVal(MyVal),
    MyFn(MyFn),
    MyTuple(MyTuple),
    MyArray(Vec<MyVar>),
}

struct MyVal {
    pub re: f64,
    pub im: f64,
}
impl From<f64> for MyVal {
    fn from(value: f64) -> Self {
        Self { re: value, im: 0.0 }
    }
}
impl Into<f64> for MyVal {
    fn into(self) -> f64 {
        self.re
    }
}
impl Into<f64> for &MyVal {
    fn into(self) -> f64 {
        self.re
    }
}
impl From<(f64, f64)> for MyVal {
    fn from(value: (f64, f64)) -> Self {
        MyVal {
            re: value.0,
            im: value.1,
        }
    }
}

struct MyTuple {
    len: usize,
    val: Vec<MyVar>,
}

struct MyFn {
    val: fn(MyTuple) -> MyVal,
}

mod my_functions;
