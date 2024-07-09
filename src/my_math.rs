use std::collections::HashMap;

use crate::{my_parser::TokenType, my_runtime::Runtime};

#[derive(Clone, Debug)]
pub enum Val {
    Re(Re),
    Im(Im),
    Fn(Fn),
    Vars(Tuple),
}
impl Val {
    pub const NONE: Self = Val::Vars(Tuple::NONE);

    pub fn reduce(self) -> Self {
        match self {
            Val::Vars(Tuple(mut v)) if v.len() == 1 => v.pop().unwrap().reduce(),
            _ => self,
        }
    }

    /// decided
    pub fn calc(&self, s: &Self, op: TokenType) -> Val {
        match op {
            TokenType::Plus => self.add(s),
            TokenType::Minus => self.sub(s),
            TokenType::Star => self.mul(s),
            TokenType::Slash => self.div(s),
            TokenType::Percent => self.rem(s),
            _ => Self::NONE,
        }
    }

    /// +
    pub fn add(&self, s: &Self) -> Val {
        match (self, s) {
            (Val::Re(r1), Val::Re(r2)) => Val::Re(Re(r1.0 + r2.0)),
            _ => Val::NONE,
        }
    }
    /// -
    pub fn sub(&self, s: &Self) -> Val {
        match (self, s) {
            (Val::Re(r1), Val::Re(r2)) => Val::Re(Re(r1.0 - r2.0)),
            _ => Val::NONE,
        }
    }
    /// *
    pub fn mul(&self, s: &Self) -> Val {
        match (self, s) {
            (Val::Re(r1), Val::Re(r2)) => Val::Re(Re(r1.0 * r2.0)),
            _ => Val::NONE,
        }
    }
    /// /
    pub fn div(&self, s: &Self) -> Val {
        match (self, s) {
            (Val::Re(r1), Val::Re(r2)) => Val::Re(Re(r1.0 / r2.0)),
            _ => Val::NONE,
        }
    }
    /// %
    pub fn rem(&self, s: &Self) -> Val {
        match (self, s) {
            (Val::Re(r1), Val::Re(r2)) => Val::Re(Re(r1.0 % r2.0)),
            _ => Val::NONE,
        }
    }

    /// -x
    pub fn neg(&self) -> Val {
        match self {
            Val::Re(r1) => Val::Re(Re(-r1.0)),
            _ => Val::NONE,
        }
    }

    /// call f(vars) where f is self
    pub fn calls(&self, vars: Val, rt: &mut Runtime) -> Val {
        match self {
            Val::Fn(f) => (f.get)(vars, rt),
            _ => Self::NONE,
        }
    }
}

#[derive(Clone, Debug)]
pub struct Re(pub f64);

#[derive(Clone, Debug)]
pub struct Im(pub f64, pub f64);

#[derive(Clone, Debug)]
pub struct Fn {
    pub get: fn(Val, &mut Runtime) -> Val,
}

#[derive(Clone, Debug)]
pub struct Tuple(pub Vec<Val>);
impl Tuple {
    pub const NONE: Self = Tuple(vec![]);
}

/// create default functions and vars
pub fn intial_system() -> HashMap<String, Val> {
    let mut res = HashMap::new();
    res
}

// impl From<f64> for MyVal {
//     fn from(value: f64) -> Self {
//         Self { re: value, im: 0.0 }
//     }
// }
// impl Into<f64> for MyVal {
//     fn into(self) -> f64 {
//         self.re
//     }
// }
// impl Into<f64> for &MyVal {
//     fn into(self) -> f64 {
//         self.re
//     }
// }
// impl From<(f64, f64)> for MyVal {
//     fn from(value: (f64, f64)) -> Self {
//         MyVal {
//             re: value.0,
//             im: value.1,
//         }
//     }
// }

// struct MyTuple {
//     len: usize,
//     val: Vec<MyVar>,
// }

// struct MyFn {
//     val: fn(MyTuple) -> MyVal,
// }

mod my_functions;
