use std::collections::HashMap;

use crate::my_math::{self, Val};

/// no clonable
pub struct Runtime {
    /// contains vars and defines.
    /// * root is indexed 1
    /// * null is indexed 0
    envs: Vec<Env>,

    /// system vars and functions
    sys_env: HashMap<String, Val>,

    current: usize,
}

impl Runtime {
    pub fn new() -> Self {
        let sys_env = my_math::intial_system();
        Runtime {
            envs: vec![Env::new(0, 0), Env::new(1, 0)],
            sys_env,
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
    pub fn get_val(&self, name: &String) -> Option<Val> {
        let mut id = self.current;
        while self.id_valid(id) {
            let e = &self.envs[id];
            if let Some(v) = e.vals.get(name) {
                return Some(v.clone());
            }
            id = e.parent;
        }
        None
    }
    /// set the val to target value
    pub fn set_val(&mut self, name: String, val: Val) {
        if !self.is_valid() {
            return;
        }
        let e = &mut self.envs[self.current];
        e.vals.insert(name, val);
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

    pub fn get_sys_val(&self, name: &String) -> Option<&Val> {
        self.sys_env.get(name)
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
