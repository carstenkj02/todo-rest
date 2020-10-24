#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use] extern crate rocket;
#[macro_use] extern crate rocket_contrib;
#[macro_use] extern crate diesel;

use serde::Serialize;
use rocket_contrib::json::Json;
use diesel::prelude::*;

mod schema;
mod models;
use models::{Task, NewTask};

#[derive(Serialize)]
struct Response {
    message: String,
}

#[get("/")]
fn list(conn: TodosDB) -> Json<Vec<Task>> {
    use crate::schema::todos::dsl::*;

    match todos.load::<models::Task>(&*conn) {
        Ok(v) => Json(v),
        Err(_) => Json(std::vec::Vec::new()),
    }
}

#[get("/<todoid>")]
fn get(conn: TodosDB, todoid: u32) -> Json<Task> {
    use crate::schema::todos::dsl::*;
    
    match todos.filter(id.eq(todoid as i32)).limit(1).load::<models::Task>(&*conn) {
        Ok(v) => Json(v[0].clone()),
        Err(_) => Json(Task{ id: -1, title: "".into() }),
    }
}

#[post("/", data = "<task_json>")]
fn add(conn: TodosDB, task_json: Json<NewTask>) -> Json<Response> {
    use schema::todos;
    let task = task_json.into_inner();
    match diesel::insert_into(todos::table).values(&task).execute(&*conn) {
        Err(e) => Json(Response{ message: format!("ERROR: {:?}", e) }),
        Ok(_) => Json(Response{ message: format!("SUCCESS: {} ADDED", task.title) }),
    }
}

#[delete("/<todoid>")]
fn delete(conn: TodosDB, todoid: i32) -> Json<Response> {
    use crate::schema::todos::dsl::*;

    match diesel::delete(todos.filter(id.eq(todoid))).execute(&*conn) {
        Err(e) => Json(Response { message: format!("ERROR: DELETING TODO: {:?}", e) }),
        Ok(_) => Json(Response { message: format!("SUCCESS: DELETED TODO {}", todoid) }),
    }
}

#[database("todos")]
struct TodosDB(diesel::SqliteConnection);

fn main() {
    rocket::ignite()
        .mount("/", routes![list, add, delete, get])
        .attach(TodosDB::fairing())
        .launch();
}
