# jodoapp_rs
A todo list app written in Rust and meant as a learning exercise. Fun!

## Summary
jodo is a command line interface (CLI) app that allows you to create, store, update, and delete todo items from your favorite (compatible) terminal.

## Usage
`jodo list`

prints out your todo list

\
`jodo add <NAME> [BODY}`

creates a todo item with name: NAME (a string), and optional body: [BODY] (also a string)

\
`jodo delete <ID>`

deletes a todo item with id: ID

\
`jodo update <ID> <NAME> [BODY]`

updates a todo item with id: ID with name: NAME, and optional body: [BODY]

\
`jodo drop`

drops your entire todo list! dangerous! there are no warnings this will wipe your list!

\
`jodo help`

prints help information. can be used with subcommands to print their help.


## TODO

- [] make output prettier with something like 'comfy-table'
- [] add a "complete"/"done" field that can be edited
- [] maybe create a TUI version -- though this seems to defeat the purpose of a quick access todo list...?
- [] allow for specification of database name/location
- [] ...
