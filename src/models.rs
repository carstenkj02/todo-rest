use super::schema::todos;
use serde::{Serialize, Deserialize};

#[derive(Serialize, Queryable, Clone)]
pub struct Task { 
    pub id: i32,
    pub title: String,
}

#[derive(Deserialize, Insertable)]
#[table_name = "todos"]
pub struct NewTask {
    pub title: String,
}
