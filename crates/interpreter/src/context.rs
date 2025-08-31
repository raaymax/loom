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

    pub fn print_stack(self: &Rc<RefCell<Context>>) {
        let context = self.borrow();
        println!("Context: {}", *context);
        for (k, v) in &context.dict {
            println!("  {}: {}", k, v);
        }
        if let Some(parent) = &context.parent {
            println!("parent:");
            parent.print_stack();
        }
        println!("---");
    }

    pub fn get_var(self: &Rc<RefCell<Context>>, name: &str) -> Option<VType> {
        let context = self.borrow();
        if let Some(v) = context.dict.get(name) {
            return Some(v.clone());
        }
        if let Some(parent) = &context.parent {
            return parent.get_var(name);
        }
        None
    }

    pub fn exists(self: &Rc<RefCell<Context>>, name: &str) -> bool {
        let context = self.borrow();
        let exists = context.dict.contains_key(name);
        if !exists {
            if let Some(parent) = &context.parent {
                parent.exists(name)
            } else { false }
        } else { true }
    }

    pub fn define_var(self: &Rc<RefCell<Context>>, name: &str, value: VType) {
        let mut context = self.borrow_mut();
        context.dict.insert(name.to_string(), value);
    }

    pub fn set_var(self: &Rc<RefCell<Context>>, name: &str, value: VType) -> Result<(), StackError> {
        let mut context = self.borrow_mut();
        if context.dict.contains_key(name){
            context.dict.insert(name.to_string(), value);
            return Ok(());
        }else{
            if let Some(parent) = &context.parent {
                println!("Var {} not found in context {} trying {}", name, context.name, parent.borrow().name);
                println!("Strong count: {} Weak count: {}", Rc::strong_count(parent), Rc::weak_count(parent));
                // Now we can access the parent context mutably through RefCell
                return parent.borrow_mut().set_var(name, value);
            } else {
                return Err(StackError::RefError);
            }
        }
    }
    pub fn create_child(self: &Rc<RefCell<Context>>, name: &str, value: VType) {
        let mut context = self.borrow_mut();
        context.dict.insert(name.to_string(), value);
    }

    pub fn populate_stack(self: &Rc<RefCell<Context>>, stack: &mut Vec<String>) {
        let context = self.borrow();
        if let Some(ref parent) = context.parent {
            parent.populate_stack(stack);
        }
        stack.push(context.to_string());
    }
}


