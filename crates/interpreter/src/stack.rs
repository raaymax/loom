use std::cell::RefCell;
use std::rc::Rc;
use lexer::Location;
use crate::vtype::VType;


pub struct StackFrame {
    function_name: String,
    location: Location,
    return_value: Option<VType>,
    return_address: Option<StackFramePtr>,
}

pub type StackFramePtr = Rc<RefCell<StackFrame>>;

impl StackFrame {
    pub fn new(function_name: &str, location: Location, return_address: Option<StackFramePtr>) -> Rc<RefCell<Self>> {
        Rc::new(RefCell::new(StackFrame {
            function_name: function_name.to_string(),
            location,
            return_value: None,
            return_address,
        }))
    }

    pub fn set_return_value(&mut self, value: VType) {
        self.return_value = Some(value);
    }

    pub fn has_returned(&self) -> bool {
        self.return_value.is_some()
    }

    pub fn get_return_value(&self) -> Option<VType> {
        self.return_value.clone()
    }

    pub fn stacktrace(&self) -> String {
        let mut trace = format!("at {}:{}", self.function_name, self.location.get_line());
        if let Some(ret_addr) = &self.return_address {
            trace.push_str(&format!("\n{}", ret_addr.borrow().stacktrace()));
        }
        trace
    }

    pub fn get_depth(&self) -> usize {
        if let Some(ret_addr) = &self.return_address {
            1 + ret_addr.borrow().get_depth()
        } else {
            1
        }
    }
}
