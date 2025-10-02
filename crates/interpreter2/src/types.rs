use std::fmt::Debug;

trait Value: Debug + Send + Sync {
    fn type_name(&self) -> &'static str;
    fn as_any(&self) -> &dyn std::any::Any;
}

trait Operator: Send + Sync {
    fn name(&self) -> &'static str;
    fn left_type(&self) -> &'static str;
    fn right_type(&self) -> &'static str;
    fn apply(&self, left: &dyn Value, right: &dyn Value) -> Result<Box<dyn Value>, String>;
}


impl Value for i32 {
    fn type_name(&self) -> &'static str {
        "i32"
    }
    fn as_any(&self) -> &dyn std::any::Any {
        self
    }
}

struct OpAdd;
impl Operator for OpAdd {
    fn name(&self) -> &'static str {
        "+"
    }
    fn left_type(&self) -> &'static str {
        "i32"
    }
    fn right_type(&self) -> &'static str {
        "i32"
    }
    fn apply(&self, left: &dyn Value, right: &dyn Value) -> Result<Box<dyn Value>, String> {
        if let (Some(l), Some(r)) = (left.as_any().downcast_ref::<i32>(), right.as_any().downcast_ref::<i32>()) {
            Ok(Box::new(l + r))
        } else {
            Err("Invalid types for Add".to_string())
        }
    }
}
