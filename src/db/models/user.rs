use db::schema::*;

#[derive(Queryable)]
pub struct User {
    pub id: i64,
    pub name: String,
}

#[derive(Insertable)]
#[table_name = "users"]
pub struct NewUser {
    pub name: String
}
