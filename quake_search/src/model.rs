use super::schema::information;

#[derive(Queryable)]
pub struct Information {
    pub id: i32,
    pub title: String,
    pub body: String,
    pub published: bool,
}

#[derive(Insertable)]
#[table_name = "information"]
pub struct NewInformation<'a> {
    pub title: &'a str,
    pub body: &'a str,
}
