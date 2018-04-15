use db::schema::*;

#[derive(Queryable)]
pub struct Entry {
    pub id: i64,
    pub url: String,
}

#[derive(Insertable)]
#[table_name = "entries"]
pub struct NewEntry {
    pub url: String,
}
