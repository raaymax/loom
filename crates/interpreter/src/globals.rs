
struct Globals;
impl Globals {
    pub fn is_builtin(name: &str) -> bool {
        match name {
            "print" => true,
            "pow" => true,
            _ => false,
        }
    }
    pub fn builtin(name: &str) -> Option<VType> {
        match name {
            "print" => Some(VType::Builtin(Builtin::Print)),
            "pow" => Some(VType::Builtin(Builtin::Pow)),
            _ => None,
        }
        
    }
}

