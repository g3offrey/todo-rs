use std::fmt::Display;

use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub(crate) struct Todo {
    pub(crate) name: String,
    pub(crate) completed: bool,
}

impl Display for Todo {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let status = if self.completed { "x" } else { " " };

        write!(f, "[{}] {}", status, self.name)
    }
}

impl Todo {
    pub(crate) fn new(name: &str) -> Self {
        Self {
            name: name.to_string(),
            completed: false,
        }
    }

    pub(crate) fn toggle_complete(&mut self) {
        self.completed = !self.completed;
    }

    pub(crate) fn match_on_name(&self, name: &str) -> bool {
        self.name.contains(name)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn toggle_complete() {
        let mut todo = Todo::new("Buy Milk");

        todo.toggle_complete();

        assert!(todo.completed);
    }

    #[test]
    fn match_on_name() {
        let todo = Todo::new("Buy Milk");
        let todo2 = Todo::new("Buy Water");

        assert!(todo.match_on_name("Milk"));
        assert!(!todo2.match_on_name("Milk"));
    }
}