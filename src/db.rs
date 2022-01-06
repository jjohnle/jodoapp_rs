use crate::todo::TodoItem;
use color_eyre::eyre::Result;
use rusqlite::{params, Connection};

pub struct Db(Connection);

pub struct TodoItemMeta {
    pub id: usize,
    pub name: String,
    pub body: Option<String>,
}

impl Db {
    // read or establish database
    pub fn open() -> Result<Db> {
        let connection = Connection::open("jodo.sqlite")?;

        connection.execute(
            "CREATE TABLE IF NOT EXISTS todoitems (
                id     INTEGER PRIMARY KEY AUTOINCREMENT,
                name   TEXT NOT NULL,
                body   TEXT 
                );",
            [],
        )?;
        Ok(Db(connection))
    }

    // little tommy drop tables the list of todo items
    pub fn drop_todo_items(&self) -> Result<()> {
        self.0.execute("DROP TABLE todoitems;", [])?;
        Ok(())
    }

    // add todo item to database
    pub fn add(&self, todo_item: &mut TodoItem) -> Result<()> {
        self.0.execute(
            "INSERT INTO todoitems (name, body) VALUES (?1, ?2)",
            params![todo_item.name, todo_item.body],
        )?;

        Ok(())
    }

    // delete todo item by id
    pub fn delete(&self, id: usize) -> Result<()> {
        self.0
            .execute("DELETE FROM todoitems WHERE id = ?1", [id])?;
        Ok(())
    }

    // generate vector of items from db for printing
    pub fn list(&self) -> Result<Vec<TodoItemMeta>> {
        let mut stmt = self
            .0
            .prepare("SELECT id, name, body FROM todoitems ORDER BY id")?;

        let todoitem_iter = stmt.query_map([], |row| {
            Ok(TodoItemMeta {
                id: row.get(0)?,
                name: row.get(1)?,
                body: row.get(2)?,
            })
        })?;

        Ok(todoitem_iter.collect::<Result<_, rusqlite::Error>>()?)
    }
}
