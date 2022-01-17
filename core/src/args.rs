use clap::{AppSettings, Parser, Subcommand};

#[derive(Parser, Debug)]
#[clap(name = "jodo")]
#[clap(about = "john's poorly engineered todo list")]
pub struct Cli {
    #[clap(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand, Debug)]
pub enum Commands {
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

    // toggle a todo item as done or not
    Mark {
        id: Option<usize>,
    },
}
