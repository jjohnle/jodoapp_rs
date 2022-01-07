use crate::db::TodoItemMeta;
use ansi_term::Color::Green;
use color_eyre::Result;
use comfy_table::modifiers::UTF8_ROUND_CORNERS;
use comfy_table::presets::UTF8_FULL;
use comfy_table::*;

pub fn create_table(todos: Vec<TodoItemMeta>) -> Result<Table> {
    let mut table = Table::new();
    table
        .load_preset(UTF8_FULL)
        .set_content_arrangement(ContentArrangement::Dynamic)
        .set_table_width(120)
        .apply_modifier(UTF8_ROUND_CORNERS)
        .set_header(vec![
            Cell::new("ID").add_attribute(Attribute::Bold),
            Cell::new("?").add_attribute(Attribute::Bold),
            Cell::new("Todo").add_attribute(Attribute::Bold),
        ]);

    for item in todos {
        let item_entry = String::from(format!(
            "{}\n{}",
            Green.paint(item.name),
            item.body.unwrap_or("".to_string())
        ));
        let item_done = if item.done {
            "true".to_string()
        } else {
            "false".to_string()
        };
        table.add_row(vec![
            Cell::new(item.id.to_string()).set_alignment(CellAlignment::Left),
            Cell::new(item_done),
            Cell::new(item_entry),
        ]);
    }

    let column = table
        .get_column_mut(0)
        .expect("Table doesn't have an ID Column??");
    column.set_cell_alignment(CellAlignment::Left);

    Ok(table)
}
