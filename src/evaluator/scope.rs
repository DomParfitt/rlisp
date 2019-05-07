use std::collections::HashMap;
use crate::evaluator::types::Type;
use std::rc::Rc;
use std::cell::RefCell;

#[derive(Debug, Clone)]
pub struct Scope {
    bindings: HashMap<String, Type>,
    parent: Option<Rc<RefCell<Scope>>>,
}

impl Scope {
    pub fn global() -> Self {
        Scope {
            bindings: HashMap::new(),
            parent: None,
        }
    }

    pub fn new(parent: RefCell<Scope>) -> Self {
        Scope {
            bindings: HashMap::new(),
            parent: Some(Rc::new(parent)),
        }
    }

    pub fn get(&self, id: &str) -> Option<Type> {
        if let Some(value) = self.bindings.get(id) {
            return Some(value.clone());
        }

        if let Some(parent) = &self.parent {
            return parent.borrow_mut().get(id);
        }

        None
    }

    pub fn bind(&mut self, id: String, value: Type) -> Type {
//        if !self.bindings.contains_key(&id) {
        self.bindings.insert(id, value.clone());
        return value;
//        } else {
//            self.get(&id).unwrap_or(Type::Err("no binding".to_string()))
//        }
    }
}