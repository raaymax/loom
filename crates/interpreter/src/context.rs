use std::{collections::HashMap, rc::Rc, fmt::Display};

use lexer::Location;

use crate::vtype::VType;

#[derive(Debug, Clone)]
pub struct Context {
    name: String,
    location: Location,
    dict: HashMap<String, VType>,
    parent: Option<Rc<Context>>,
}

impl Display for Context {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}:{}", self.name, self.location.get_line())
    }
}

impl Context {
    pub fn new(name: &str, location: Location) -> Rc<Self> {
        Rc::new(Context {
            name: name.to_string(),
            location,
            dict: HashMap::new(),
            parent: None,
        })
    }
    pub fn new_child(name: &str, location: Location, parent: &Rc<Context>) -> Rc<Self> {
        Rc::new(Context {
            name: name.to_string(),
            location,
            dict: HashMap::new(),
            parent: Some(parent.clone()),
        })
    }

    pub fn get_var(self: &Rc<Context>, name: &str) -> Option<VType> {
        if let Some(v) = self.dict.get(name) {
            return Some(v.clone());
        }
        if let Some(parent) = &self.parent {
            return parent.get_var(name);
        }
        None
    }
    pub fn set_var(&mut self, name: &str, value: VType) {
        self.dict.insert(name.to_string(), value);
    }
    pub fn create_child(&mut self, name: &str, value: VType) {
        self.dict.insert(name.to_string(), value);
    }

    pub fn populate_stack(&self, stack: &mut Vec<String>) {
        if let Some(ref parent) = self.parent {
            parent.populate_stack(stack);
        }
        stack.push(self.to_string());
    }
}


