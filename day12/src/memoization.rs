use std::hash::Hash;
use std::collections::HashMap;

pub struct Memoization<I: Hash + Eq + Clone, O: Clone, F: Fn(I) -> O> {
    data: HashMap<I, O>,
    lambda: Box<F>,
}
impl<I: Hash + Eq + Clone, O: Clone, F: Fn(I) -> O> Memoization<I, O, F> {
    pub fn new(lambda: F) -> Memoization<I, O, F> {
        Memoization {
            data: HashMap::new(),
            lambda: Box::new(lambda),
        }
    }

    pub fn exec(&mut self, arg: I) -> O {
        match self.data.get(&arg) {
            Option::Some(val) => return val.clone(),
            Option::None => {}
        };
        let ret = (self.lambda)(arg.clone());
        let _ = self.data.insert(arg, ret.clone());
        ret
    }
}
