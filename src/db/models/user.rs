use db::schema::users;

#[derive(Queryable)]
pub struct User {
    pub id: i64,
    pub name: String,
}


impl User {
    pub fn next_by_name(user_name: String, playlist_name: String) {

    }
}

#[derive(Insertable)]
#[table_name = "users"]
pub struct NewUser {
    pub name: String
}
