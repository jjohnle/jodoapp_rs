#![allow(dead_code)]
#![allow(unused_variables)]

mod db;
mod todo;

use clap::{AppSettings, Parser, Subcommand};
use color_eyre::eyre::Result;
use db::Db;
use todo::TodoItem;

#[derive(Parser, Debug)]
#[clap(name = "jodo")]
#[clap(about = "john's poorly engineered todo list")]
struct Cli {
    #[clap(subcommand)]
    command: Commands,
}

#[derive(Subcommand, Debug)]
enum Commands {
    // list all current to do items
    List {},

    // add a todo item
    #[clap(setting(AppSettings::ArgRequiredElseHelp))]
    Add {
        name: String,
        body: Option<String>,
    },

    // delete a todo item
    #[clap(setting(AppSettings::ArgRequiredElseHelp))]
    Delete {
        id: usize,
    },

    // update a todo item
    #[clap(setting(AppSettings::ArgRequiredElseHelp))]
    Update {
        id: usize,
        name: String,
        body: Option<String>,
    },

    // danger !!!!!! drop all todo items from list
    Drop {},
}

fn main() -> Result<()> {
    color_eyre::install()?;
    let args = Cli::parse();
    let db = Db::open()?;

    match args.command {
        Commands::List {} => list(db)?,
        Commands::Add { name, body } => {
            let new_todo_item = TodoItem::new(name, body);
            db.add(&mut new_todo_item.unwrap())?;
            list(db)?
        }
        Commands::Delete { id } => {
            db.delete(id)?;
            list(db)?
        }
        Commands::Update { id, name, body } => {
            println!("id: {}, name: {:?}, body: {:?}", id, &name, body);
            let updated_body = match body {
                Some(body) => body,
                None => db
                    .get(id)?
                    .body
                    .expect("Problem getting todo item {}'s body!"),
            };

            let updated_todo_item = TodoItem {
                id: Some(id),
                name,
                body: Some(updated_body),
            };

            db.update(updated_todo_item)?;
            list(db)?
        }
        Commands::Drop {} => {
            db.drop_todo_items()?;
            println!("Dropped table!");
        }
    };

    Ok(())
}

fn list(db: Db) -> Result<()> {
    println!("{:5} {:30} {:30}", "id", "name", "body");
    for entry in db.list()? {
        println!(
            "{:5} {:30} {:30}",
            entry.id,
            entry.name,
            entry.body.unwrap_or_else(|| String::from("")),
        )
    }
    Ok(())
}
