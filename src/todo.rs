use color_eyre::eyre::Result;
use std::fmt::{self, Display};

// struct that represents a todo item
#[derive(Debug, Clone)]
pub struct TodoItem {
    pub id: Option<usize>,
    pub name: String,
    pub body: Option<String>,
    pub done: bool,
}

impl TodoItem {
    pub fn new(name: String, body: Option<String>) -> Result<TodoItem> {
        Ok(TodoItem {
            id: None,
            name,
            body,
            done: false,
        })
    }
}

impl Display for TodoItem {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}: {}", self.id.unwrap(), self.name)
    }
}
