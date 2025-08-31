use std::{collections::HashMap, fmt::Display, rc::Rc, cell::RefCell};

use lexer::{Location};

use crate::vtype::VType;

pub enum StackError {
    ValueError,
    RefError,
}

#[derive(Debug, Clone)]
pub struct Context {
    name: String,
    location: Location,
    dict: HashMap<String, VType>,
    parent: Option<Rc<RefCell<Context>>>,
}



impl Display for Context {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}:{}", self.name, self.location.get_line())
    }
}

impl Context {
    pub fn new(name: &str, location: Location) -> Rc<RefCell<Self>> {
        Rc::new(RefCell::new(Context {
            name: name.to_string(),
            location,
            dict: HashMap::new(),
            parent: None,
        }))
    }
    pub fn new_child(name: &str, location: Location, parent: &Rc<RefCell<Context>>) -> Rc<RefCell<Self>> {
        Rc::new(RefCell::new(Context {
            name: format!("{}:{}", parent.borrow().name, name),
            location,
            dict: HashMap::new(),
            parent: Some(parent.clone()),
        }))
    }

    pub fn print_stack(&self) {
        println!("Context: {}", self);
        for (k, v) in &self.dict {
            println!("  {}: {}", k, v);
        }
        if let Some(parent) = &self.parent {
            println!("parent:");
            parent.borrow().print_stack();
        }
        println!("---");
    }

    pub fn get_var(&self, name: &str) -> Option<VType> {
        if let Some(v) = self.dict.get(name) {
            return Some(v.clone());
        }
        if let Some(parent) = &self.parent {
            return parent.borrow().get_var(name);
        }
        None
    }

    pub fn exists(&self, name: &str) -> bool {
        let exists = self.dict.contains_key(name);
        if !exists {
            if let Some(parent) = &self.parent {
                parent.borrow().exists(name)
            } else { false }
        } else { true }
    }

    pub fn define_var(&mut self, name: &str, value: VType) {
        self.dict.insert(name.to_string(), value);
    }

    pub fn set_var(&mut self, name: &str, value: VType) -> Result<(), StackError> {
        if self.dict.contains_key(name){
            self.dict.insert(name.to_string(), value);
            return Ok(());
        }else{
            if let Some(parent) = &self.parent {
                return parent.borrow_mut().set_var(name, value);
            } else {
                return Err(StackError::RefError);
            }
        }
    }
    pub fn create_child(&mut self, name: &str, value: VType) {
        self.dict.insert(name.to_string(), value);
    }

    pub fn populate_stack(&self, stack: &mut Vec<String>) {
        if let Some(ref parent) = self.parent {
            parent.borrow().populate_stack(stack);
        }
        stack.push(self.to_string());
    }
}


