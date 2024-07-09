use std::collections::HashMap;

use crate::{
    my_parser::TokenType,
    my_runtime::{Runtime, System},
};

pub(crate) mod my_functions;

#[derive(Clone, Debug)]
pub enum Val {
    Re(Re),
    Im(Im),
    Fun(Fun),
    Vars(Tuple),
}
impl Val {
    pub fn print(&self, index: &mut usize) -> String {
        match self {
            Val::Re(Re(v)) => {
                *index += 1;
                format!("[out {index}] {v}\n")
            }
            Val::Im(Im(u, v)) => {
                *index += 1;
                format!("[out {index}] {u}+{v}i\n")
            }
            Val::Fun(Fun::System(name)) => {
                *index += 1;
                format!("[out {index}] @fun: {name}\n")
            }
            Val::Vars(t) => t.print(index),
        }
    }
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
    pub fn calls(&self, rt: &mut Runtime, vars: Val) -> Val {
        match self {
            Val::Fun(Fun::System(name)) => System::calls(rt, name, vars),
            _ => Self::NONE,
        }
    }
}

#[derive(Clone, Debug)]
pub struct Re(pub f64);

#[derive(Clone, Debug)]
pub struct Im(pub f64, pub f64);

#[derive(Clone, Debug)]
pub enum Fun {
    System(String),
}

#[derive(Clone, Debug)]
pub struct Tuple(pub Vec<Val>);
impl Tuple {
    pub const NONE: Self = Tuple(vec![]);
}

impl Tuple {
    fn print(&self, index: &mut usize) -> String {
        let mut res = String::new();
        for v in &self.0 {
            res += &v.print(index);
        }
        res
    }
}

/// create default functions and vars
pub fn intial_system() -> HashMap<String, Val> {
    let mut res = HashMap::new();
    res
}
