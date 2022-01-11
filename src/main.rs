mod cli;
mod db;
mod dialogue;
mod table;
mod todo;

use clap::Parser;
use cli::{Cli, Commands};
use color_eyre::eyre::Result;
use db::Db;
use table::create_table;
use todo::TodoItem;

fn main() -> Result<()> {
    color_eyre::install()?;
    let args = Cli::parse();
    let db = Db::open()?;

    match args.command {
        Commands::List {} => {
            println!("{}", create_table(db.list()?)?);
            // list(db)?
        }
        Commands::Add { name, body } => {
            let new_todo_item = TodoItem::new(name, body);
            db.add(&mut new_todo_item.unwrap())?;
            println!("{}", create_table(db.list()?)?);
            // list(db)?
        }
        Commands::Delete { id } => {
            db.delete(id)?;
            println!("{}", create_table(db.list()?)?);
            // list(db)?
        }
        Commands::Update { id, name, body } => {
            let updated_body = match body {
                Some(body) => body,
                None => db.get(id)?.body.expect("Problem getting todo item body!"),
            };

            let updated_todo_item = TodoItem {
                id: Some(id),
                name,
                body: Some(updated_body),
                done: db.get(id)?.done,
            };

            db.update(updated_todo_item)?;
            println!("{}", create_table(db.list()?)?);
            // list(db)?
        }
        Commands::Drop {} => {
            db.drop_todo_items()?;
            println!("Dropped table!");
        }
        Commands::Mark { id } => match id {
            Some(id) => {
                db.toggle_done(id)?;
                println!("{}", create_table(db.list()?)?);
            }
            None => {
                let todos = db.list().unwrap();
                let chosen = dialogue::single_dialogue(todos.clone())?;
                db.toggle_done(todos.get(chosen.unwrap()).unwrap().id.unwrap())?;
                println!("{}", create_table(db.list()?)?);
            }
        },
    };

    Ok(())
}

// fn list(db: Db) -> Result<()> {
//     println!("{:5} {:30} {:30}", "id", "name", "body");
//     for entry in db.list()? {
//         println!(
//             "{:5} {:30} {:30}",
//             entry.id,
//             entry.name,
//             entry.body.unwrap_or_else(|| String::from("")),
//         )
//     }
//     Ok(())
// }
