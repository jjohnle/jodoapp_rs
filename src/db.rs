use crate::todo::TodoItem;
use color_eyre::eyre::Result;
use rusqlite::{params, Connection};
pub struct Db(Connection);

impl Db {
    // read or establish database
    pub fn open() -> Result<Db> {
        let connection = Connection::open("jodo.sqlite")?;

        connection.execute(
            "CREATE TABLE IF NOT EXISTS todoitems (
                id     INTEGER PRIMARY KEY AUTOINCREMENT,
                name   TEXT NOT NULL,
                body   TEXT,
                done   BOOL NOT NULL
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
            "INSERT INTO todoitems (name, body, done) VALUES (?1, ?2, ?3)",
            params![todo_item.name, todo_item.body, todo_item.done],
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
    pub fn list(&self) -> Result<Vec<TodoItem>> {
        let mut stmt = self
            .0
            .prepare("SELECT id, name, body, done FROM todoitems ORDER BY id")?;

        let todoitem_iter = stmt.query_map([], |row| {
            Ok(TodoItem {
                id: row.get(0)?,
                name: row.get(1)?,
                body: row.get(2)?,
                done: row.get(3)?,
            })
        })?;

        Ok(todoitem_iter.collect::<Result<_, rusqlite::Error>>()?)
    }

    // update a task based on ID
    pub fn update(&self, todo_item_update: TodoItem) -> Result<()> {
        self.0.execute(
            "UPDATE todoitems SET name = ?1, body = ?2 where id = ?3",
            params![
                todo_item_update.name,
                todo_item_update.body,
                todo_item_update.id
            ],
        )?;

        Ok(())
    }

    // get a task based on ID
    pub fn get(&self, id: usize) -> Result<TodoItem> {
        let mut stmt = self
            .0
            .prepare("SELECT id, name, body, done FROM todoitems WHERE id = ?1")?;

        let to_ret = stmt.query_row(params![id], |row| {
            Ok(TodoItem {
                id: row.get(0)?,
                name: row.get(1)?,
                body: row.get(2).unwrap_or_else(|_| None),
                done: row.get(3)?,
            })
        });

        Ok(to_ret?)
    }

    pub fn toggle_done(&self, id: usize) -> Result<()> {
        let curr_done = self.get(id)?.done;
        self.0.execute(
            "UPDATE todoitems SET done = ?1 WHERE id = ?2",
            params![!curr_done, id],
        )?;

        Ok(())
    }
}
