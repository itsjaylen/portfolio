use diesel::prelude::*;
use super::schema::tasks;

#[derive(Queryable, Selectable, Identifiable, AsChangeset)]
#[diesel(table_name = crate::schema::tasks)]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
#[derive(Debug)]
pub struct Task {
    pub id: Option<i32>,
    pub title: String,
    pub description: Option<String>,
    pub created_at: Option<String>, // Change the type based on how you want to handle timestamps
    pub completed: bool,
    pub completed_at: Option<String>, // Change the type based on how you want to handle timestamps
}



#[derive(Insertable, Default)]
#[diesel(table_name = tasks)]
pub struct NewTask<'a> {
    pub title: &'a str,
    pub description: &'a str,
}

impl<'a> NewTask<'a> {
    pub fn new(title: &'a str, description: &'a str) -> Self {
        NewTask {
            title,
            description,
            ..Default::default()
        }
    }
}

#[derive(AsChangeset)]
#[diesel(table_name = tasks)]
pub struct UpdatedTask {
    pub title: Option<String>,
    pub description: Option<String>,
    pub completed: Option<bool>,
    pub completed_at: Option<String>,
}
