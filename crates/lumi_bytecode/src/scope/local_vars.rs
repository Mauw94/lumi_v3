use std::collections::HashMap;

pub trait ScopeManager {
    fn get_or_create_local(&mut self, name: &str) -> usize;
    fn get_local(&self, name: &str) -> Option<&usize>;
}

pub trait ScopeCore {
    fn local_vars(&self) -> &HashMap<String, usize>;
    fn local_vars_mut(&mut self) -> &mut HashMap<String, usize>;
    fn next_local(&self) -> usize;
    fn set_next_local(&mut self, next: usize);
}

impl<T> ScopeManager for T
where
    T: ScopeCore,
{
    fn get_or_create_local(&mut self, name: &str) -> usize {
        if let Some(&idx) = self.local_vars().get(name) {
            idx
        } else {
            let idx = self.next_local();
            self.local_vars_mut().insert(name.to_string(), idx);
            self.set_next_local(self.next_local() + 1);
            idx
        }
    }

    fn get_local(&self, name: &str) -> Option<&usize> {
        self.local_vars().get(name)
    }
}
