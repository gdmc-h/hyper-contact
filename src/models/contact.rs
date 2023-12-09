use diesel::{deserialize::Queryable, Selectable, Insertable};

#[derive(Clone, Queryable, Selectable, Debug)]
#[diesel(table_name = crate::schema::contacts)]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
pub struct Contact {
    pub id: i32,
    pub first: String,
    pub last: String,
    pub phone: String,
    pub email: String
}

#[derive(Insertable, Deserialize, Serialize)]
#[diesel(table_name = crate::schema::contacts)]
pub struct NewContact {
    pub first: String,
    pub last: String,
    pub phone: String,
    pub email: String,
}