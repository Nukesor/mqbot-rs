use db::schema::users;

#[derive(Queryable)]
pub struct User {
    pub name: String,
}

impl User {}

#[derive(Insertable)]
#[table_name = "users"]
pub struct NewUser {
    pub name: String,
}
