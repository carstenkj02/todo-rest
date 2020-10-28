# TODO-REST
A very simple web server for keeping track of todos.
* Keeps them in a sqlite db.
* Exposes a rest api for adding, deleting, and getting todos.
* Enables tera template rendering for viewing todos.

## Install
Build the application with
```
cargo build --release
```
The binary can there after be found in `target/release/todo-rest`.

The database, and template location, port, and other options can be set in `Rocket.toml`, the `Rocket.toml` must be in the same directory as the binary is launched from.

Theres no table creation in the application so you can use `todo.db.ready` as a starter.

## Making new themes
1. Create a new folder in the templates directory with the name of your theme
1. Create a `index.html.tera` or `index.tera` in the folder.
1. This will now be accesible at `URL/views/<THEME>`

Tasks are passed as a list with the name `tasks`.
