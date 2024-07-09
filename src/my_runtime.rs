use std::collections::HashMap;

use crate::my_math::{Fun, Re, Tuple, Val};

/// no clonable
pub struct Runtime {
    /// contains vars and defines.
    /// * root is indexed 1
    /// * null is indexed 0
    envs: Vec<Env>,

    /// system vars and functions
    pub sys: System,

    current: usize,
}

impl Runtime {
    pub fn new() -> Self {
        let sys = System::new();
        Runtime {
            envs: vec![Env::new(0, 0), Env::new(1, 0)],
            sys,
            current: 1,
        }
    }
    /// check validity
    pub fn is_valid(&self) -> bool {
        self.current != 0 && self.current < self.envs.len()
    }
    /// check validity
    pub fn id_valid(&self, id: usize) -> bool {
        id != 0 && id < self.envs.len()
    }
    /// insert a new env
    pub fn push(&mut self) -> usize {
        if !self.is_valid() {
            return 0;
        }
        let id = self.envs.len();
        let parent = self.current;
        self.envs.push(Env::new(id, parent));
        self.current = id;
        id
    }
    /// leave this env
    pub fn pop(&mut self) -> usize {
        if !self.is_valid() {
            return 0;
        }
        let id = self.envs[self.current].parent;
        if !self.id_valid(id) {
            return 0;
        }
        self.current = id;
        return id;
    }
    /// restart kernel
    pub fn restart(&mut self) {
        self.current = 0;
        self.envs.clear();
        self.envs.push(Env::new(0, 0));
        self.envs.push(Env::new(1, 0));
        self.current = 1;
    }
    /// clear all vals
    pub fn clear_all(&mut self) {
        self.envs.iter_mut().for_each(|e| e.clear());
    }
    /// clear current vals
    pub fn clear(&mut self) {
        if self.is_valid() {
            self.envs.clear();
        }
    }

    /// get a copy of most relative val from current env
    pub fn get_val(&self, name: &String) -> Val {
        let mut id = self.current;
        while self.id_valid(id) {
            let e = &self.envs[id];
            if let Some(v) = e.vals.get(name) {
                return v.to_owned();
            }
            id = e.parent;
        }
        self.get_sys_val(name)
    }
    /// set the val to target value
    pub fn set_val(&mut self, name: String, val: Val) {
        let mut id = self.current;
        while self.id_valid(id) {
            let e = &mut self.envs[id].vals;
            if e.contains_key(&name) {
                e.insert(name, val);
                return;
            }
            id = self.envs[id].parent;
        }
        if !self.is_valid() {
            return;
        }
        self.envs[self.current].vals.insert(name, val);
    }
    /// copy all related vars
    pub fn copy_env(&self) -> HashMap<String, Val> {
        self.copy_env_id(self.current)
    }
    /// copy all related vars of some id
    pub fn copy_env_id(&self, id: usize) -> HashMap<String, Val> {
        let mut id = id;
        let mut res = HashMap::new();
        while self.id_valid(id) {
            let e = &self.envs[id].vals;
            for (k, v) in e.iter() {
                if !res.contains_key(k) {
                    res.insert(k.to_owned(), v.to_owned());
                }
            }
        }
        res
    }

    pub fn get_sys_val(&self, name: &String) -> Val {
        if let Some(val) = self.sys.get_sys_val(name) {
            val.to_owned()
        } else {
            Val::Fun(self.sys.get_sys_fun(name))
        }
    }

    /// try to call the fun
    pub fn calls(&mut self, name: &String, vars: Val) -> Val {
        let mut id = self.current;
        while self.id_valid(id) {
            let e = &self.envs[id];
            if let Some(v) = e.vals.get(name) {
                return v.clone().calls(self, vars);
            }
            id = e.parent;
        }
        System::calls(self, name, vars)
    }
}

/// no clonable
pub struct Env {
    // relations
    id: usize,
    parent: usize,
    children: Vec<usize>,
    // vals
    vals: HashMap<String, Val>,
}

impl Env {
    pub fn new(id: usize, parent: usize) -> Self {
        Env {
            id,
            parent,
            children: Vec::new(),
            vals: HashMap::new(),
        }
    }
    /// get a copy of current env
    /// * the vars outside it will be alternatively copied
    pub fn get_copy(&self, runtime: &Runtime) -> HashMap<String, Val> {
        runtime.copy_env_id(self.id)
    }
    pub fn clear(&mut self) {
        self.vals.clear();
    }
}

pub struct System {
    // constants
    vals: HashMap<String, Val>,

    // statics
    epsilon_: f64,
}

impl System {
    fn new() -> Self {
        Self {
            vals: HashMap::new(),
            epsilon_: 1.0e-9f64,
        }
    }

    pub fn get_sys_val(&self, name: &String) -> Option<&Val> {
        self.vals.get(name)
    }
    pub fn get_sys_fun(&self, name: &String) -> Fun {
        Fun::System(name.clone())
    }

    pub fn calls(rt: &mut Runtime, name: &String, vars: Val) -> Val {
        match vars.reduce() {
            Val::Re(v) => Self::call_Re1(rt, name, v),
            Val::Vars(Tuple(mut vars)) if vars.len() == 2 => {
                let v2 = vars.pop().unwrap();
                let v1 = vars.pop().unwrap();
                match (v1, v2) {
                    (Val::Re(v1), Val::Re(v2)) => Self::call_Re2(rt, name, v1, v2),
                    _ => Val::NONE,
                }
            }
            _ => Val::NONE,
        }
    }

    fn call_Re1(rt: &mut Runtime, name: &str, v: Re) -> Val {
        match name {
            "absolute" | "abs" => Val::Re(Re(Self::_abs(v.0))),
            "negative" | "neg" => Val::Re(Re(Self::_neg(v.0))),
            "round" => Val::Re(Re(Self::_round(v.0))),
            "ceil" => Val::Re(Re(Self::_ceil(v.0))),
            "floor" | "int" => Val::Re(Re(Self::_floor(v.0))),
            "sin" => Val::Re(Re(Self::_sin(v.0))),
            "cos" => Val::Re(Re(Self::_cos(v.0))),
            "tan" => Val::Re(Re(Self::_tan(v.0))),
            "cot" => Val::Re(Re(Self::_cot(v.0))),
            "sec" => Val::Re(Re(Self::_sec(v.0))),
            "csc" => Val::Re(Re(Self::_csc(v.0))),
            "asin" | "arcsin" => Val::Re(Re(Self::_arcsin(v.0))),
            "acos" | "arccos" => Val::Re(Re(Self::_arccos(v.0))),
            "atan" | "arctan" => Val::Re(Re(Self::_arctan(v.0))),
            "acot" | "arccot" => Val::Re(Re(Self::_arccot(v.0))),
            "asec" | "arcsec" => Val::Re(Re(Self::_arcsec(v.0))),
            "acsc" | "arccsc" => Val::Re(Re(Self::_arccsc(v.0))),
            "sinh" => Val::Re(Re(Self::_sinh(v.0))),
            "cosh" => Val::Re(Re(Self::_cosh(v.0))),
            "tanh" => Val::Re(Re(Self::_tanh(v.0))),
            "coth" => Val::Re(Re(Self::_coth(v.0))),
            "sech" => Val::Re(Re(Self::_sech(v.0))),
            "csch" => Val::Re(Re(Self::_csch(v.0))),
            "asinh" | "arcsinh" => Val::Re(Re(Self::_arcsinh(v.0))),
            "acosh" | "arccosh" => Val::Re(Re(Self::_arccosh(v.0))),
            "atanh" | "arctanh" => Val::Re(Re(Self::_arctanh(v.0))),
            "acoth" | "arccoth" => Val::Re(Re(Self::_arccoth(v.0))),
            "asech" | "arcsech" => Val::Re(Re(Self::_arcsech(v.0))),
            "acsch" | "arccsch" => Val::Re(Re(Self::_arccsch(v.0))),
            "raddegree" | "todegree" => Val::Re(Re(Self::_rad_to_deg(v.0))),
            "degreerad" | "torad" => Val::Re(Re(Self::_deg_to_rad(v.0))),
            "square" => Val::Re(Re(Self::_square(v.0))),
            "cube" => Val::Re(Re(Self::_cube(v.0))),
            "sqrt" | "sqr" => Val::Re(Re(Self::_sqrt(v.0))),
            "cbrt" | "cbr" => Val::Re(Re(Self::_cbrt(v.0))),
            "exp" => Val::Re(Re(Self::_exp(v.0))),
            "log10" => Val::Re(Re(Self::_log10(v.0))),
            "loge" | "ln" | "log" => Val::Re(Re(Self::_ln(v.0))),
            "log2" => Val::Re(Re(Self::_log2(v.0))),
            // need instance
            "zero" => Val::Re(Re(rt.sys._iszero(v.0) as i32 as f64)),
            _ => Val::NONE,
        }
    }
    fn call_Re2(rt: &mut Runtime, name: &str, v1: Re, v2: Re) -> Val {
        match name {
            "add" | "plus" => Val::Re(Re(Self::_add(v1.0, v2.0))),
            "substract" | "minus" => Val::Re(Re(Self::_sub(v1.0, v2.0))),
            "multiply" | "dot" => Val::Re(Re(Self::_mulitply(v1.0, v2.0))),
            "devide" | "frac" => Val::Re(Re(Self::_devide(v1.0, v2.0))),
            "arctan2" | "atan2" | "arctan" | "atan" => Val::Re(Re(Self::_arctan2(v1.0, v2.0))),
            "pow" | "power" => Val::Re(Re(Self::_pow(v1.0, v2.0))),
            "log" | "logarithm" => Val::Re(Re(Self::_log(v1.0, v2.0))),
            _ => Val::NONE,
        }
    }
}

use crate::my_math::my_functions::SysFunctionReal;

impl SysFunctionReal for System {
    fn _get_epsilon_(&self) -> f64 {
        self.epsilon_
    }
}
