use color_eyre::Result;
use dialoguer::{theme::ColorfulTheme, Select};

use crate::TodoItem;

// takes an item or vector of todo items and adds to dialogue list
// returns an option with Some(index of item chosen) or None if user cancelled

pub fn single_dialogue(items: Vec<TodoItem>) -> Result<Option<usize>> {
    let selection = Select::with_theme(&ColorfulTheme::default())
        .items(&items)
        .interact_opt()?;

    Ok(selection)
}
