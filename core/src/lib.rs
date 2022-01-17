mod db;
mod todo;

use db::Db;
use napi::{Error, Result};
use napi_derive::napi;
use todo::TodoItem;

#[napi]
pub struct UiState {
    db: Db,
}

#[napi]
pub struct JsTodoItem(TodoItem);

#[napi]
impl JsTodoItem {
    #[napi(getter)]
    pub fn get_id(&self) -> usize {
        self.0.id.unwrap()
    }

    #[napi(getter)]
    pub fn get_name(&self) -> String {
        self.0.name.clone()
    }

    #[napi(getter)]
    pub fn get_body(&self) -> String {
        self.0.body.as_ref().unwrap().to_string()
    }

    #[napi(getter)]
    pub fn get_done(&self) -> bool {
        self.0.done
    }
}

impl From<TodoItem> for JsTodoItem {
    fn from(todo_item: TodoItem) -> Self {
        JsTodoItem(todo_item)
    }
}

#[napi]
impl UiState {
    #[napi(constructor)]
    pub fn new() -> Result<UiState> {
        Ok(UiState {
            db: Db::open().map_err(|e| Error::from_reason(e.to_string()))?,
        })
    }

    #[napi]
    pub fn get_todo_items(&self) -> Result<Vec<JsTodoItem>> {
        self.db
            .list()
            .map_err(|e| Error::from_reason(e.to_string()))
            .map(|todos| todos.into_iter().map(JsTodoItem::from).collect())
    }
}
