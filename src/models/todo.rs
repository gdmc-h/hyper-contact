use diesel::sql_types::Bool;

pub struct Todo {
    pub id: i32,
    pub title: String,
    pub done: Bool
}