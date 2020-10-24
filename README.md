# TODO-REST
A very simple web server for keeping track of todos, keeps them in a sqlite db, and exposes a rest api for add, deleting, and getting.

## Install
Build the application with
```
cargo build --release
```
The binary can be found in `target/release/todo-rest`.

The database location can be set in `Rocket.toml` and theres no table creation in the application so you can use `todo.db.ready` as a starter
