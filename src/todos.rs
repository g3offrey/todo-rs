use std::error::Error;
use std::fmt::Display;

use anyhow::anyhow;

use crate::todo::Todo;

const FILE_NAME: &str = "/tmp/data.mpk";

pub(crate) struct Todos {
    list: Vec<Todo>,
}

impl Todos {
    pub(crate) fn new() -> Self {
        Self { list: Vec::new() }
    }

    pub(crate) fn add_todo(&mut self, name: &str) {
        self.list.push(Todo::new(name));
    }

    pub(crate) fn delete_todo_by_name(&mut self, name: &str) {
        self.list = self.list.drain(..).filter(|todo| !todo.match_on_name(name)).collect();
    }

    pub(crate) fn delete_todo_by_index(&mut self, index_to_delete: usize) -> Result<(), anyhow::Error> {
        if index_to_delete == 0 {
            return Err(anyhow!("Index must be greater than 0"));
        }

        if self.list.len() < index_to_delete {
            return Ok(());
        }

        self.list = self
            .list
            .drain(..)
            .enumerate()
            .filter(|(index, _)| *index != index_to_delete - 1)
            .map(|(_, todo)| todo)
            .collect();

        Ok(())
    }

    pub(crate) fn display_in_console(&self) {
        println!("### Todo list ###");
        println!("{}", self);
    }

    pub(crate) fn complete_by_index(&mut self, index: usize) -> Result<(), anyhow::Error> {
        if index == 0 {
            return Err(anyhow!("Index must be greater than 0"));
        }

        if self.list.len() < index {
            return Ok(());
        }

        self.list[index - 1].toggle_complete();

        Ok(())
    }

    pub(crate) fn complete_by_name(&mut self, search: &str) {
        for todo in &mut self.list {
            if todo.match_on_name(search) {
                todo.toggle_complete();
                break;
            }
        }
    }

    pub(crate) fn export(&self) -> Result<(), Box<dyn Error>> {
        let content = rmp_serde::to_vec_named(&self.list)?;

        std::fs::write(FILE_NAME, content)?;

        Ok(())
    }

    pub(crate) fn import(&mut self) -> Result<(), Box<dyn Error>> {
        let read_file = std::fs::read(FILE_NAME);

        match read_file {
            Ok(content) => {
                self.list = rmp_serde::from_slice(&content)?;
            }
            Err(_) => {
                self.export()?;
            }
        }

        Ok(())
    }
}

impl Display for Todos {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut output = String::new();

        for (index, todo) in self.list.iter().enumerate() {
            output.push_str(&format!("{}. {}\n", index + 1, todo));
        }

        write!(f, "{}", output)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn display() {
        let mut todos = Todos::new();
        todos.add_todo("Learn Rust");
        todos.add_todo("Profit!");

        assert_eq!(format!("{}", todos), "1. [ ] Learn Rust\n2. [ ] Profit!\n");
    }

    #[test]
    fn add_todo() {
        let mut todos = Todos::new();
        todos.add_todo("Learn Rust");
        todos.add_todo("Profit!");

        assert_eq!(todos.list.len(), 2);
        assert_eq!(todos.list[0].name, "Learn Rust");
        assert_eq!(todos.list[1].name, "Profit!");
        assert!(!todos.list[0].completed);
        assert!(!todos.list[1].completed);
    }

    #[test]
    fn complete_by_name() {
        let mut todos = Todos::new();
        todos.add_todo("Learn Rust");
        todos.add_todo("Profit!");

        todos.complete_by_name("Rust");

        assert!(todos.list[0].completed);
        assert!(!todos.list[1].completed);
    }

    #[test]
    fn complete_by_index() {
        let mut todos = Todos::new();
        todos.add_todo("Learn Rust");
        todos.add_todo("Profit!");

        todos.complete_by_index(1).unwrap();

        assert!(todos.list[0].completed);
        assert!(!todos.list[1].completed);
    }

    #[test]
    fn uncompleted_by_name() {
        let mut todos = Todos::new();
        todos.add_todo("Learn Rust");
        todos.add_todo("Profit!");

        todos.complete_by_name("Rust");
        todos.complete_by_name("Rust");

        assert!(!todos.list[0].completed);
        assert!(!todos.list[1].completed);
    }

    #[test]
    fn uncompleted_by_index() {
        let mut todos = Todos::new();
        todos.add_todo("Learn Rust");
        todos.add_todo("Profit!");

        todos.complete_by_index(1).unwrap();
        todos.complete_by_index(1).unwrap();

        assert!(!todos.list[0].completed);
        assert!(!todos.list[1].completed);
    }

    #[test]
    fn delete_by_name() {
        let mut todos = Todos::new();
        todos.add_todo("Learn Rust");
        todos.add_todo("Profit!");

        todos.delete_todo_by_name("Learn Rust");

        assert_eq!(todos.list.len(), 1);
        assert_eq!(todos.list[0].name, "Profit!");
    }

    #[test]
    fn delete_by_index() {
        let mut todos = Todos::new();
        todos.add_todo("Learn Rust");
        todos.add_todo("Profit!");

        todos.delete_todo_by_index(1).unwrap();

        assert_eq!(todos.list.len(), 1);
        assert_eq!(todos.list[0].name, "Profit!");
    }
}