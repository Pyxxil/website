use serde::Serialize;

#[derive(Queryable, Serialize, Debug)]
pub struct Post {
    pub id: i32,
    pub title: String,
    pub body: String,
    pub date: String,
    pub reading_time: i32,
    pub summary: String,
}

#[derive(Queryable, Serialize, Debug)]
pub struct Project {
    pub id: i32,
    pub title: String,
    pub link: String,
    pub description: String,
}
