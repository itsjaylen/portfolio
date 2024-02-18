use diesel::prelude::*;
use super::schema::tasks;

// The task model for the database referenced to schema
#[derive(Queryable, Selectable, Identifiable, AsChangeset)]
#[diesel(table_name = crate::schema::tasks)]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
#[derive(Debug)]
pub struct Task {
    pub id: Option<i32>,
    pub title: String,
    pub description: Option<String>,
    pub created_at: Option<String>,
    pub completed: bool,
    pub completed_at: Option<String>,
}

// New task model used to create a new task in the database
// Ensures that all variables have the same lifetime ('a)
#[derive(Insertable, Default)]
#[diesel(table_name = tasks)]
pub struct NewTask<'a> {
    pub title: &'a str,
    pub description: &'a str,
}

impl<'a> NewTask<'a> {
    // Constructor for creating a new NewTask instance
    // Takes two references with the same lifetime ('a)
    pub fn new(title: &'a str, description: &'a str) -> Self {
        // Creates a new NewTask instance with the given title and description
        // Default none the other values for later changes in the database
        NewTask {
            title,
            description,
            ..Default::default()
        }
    }
}

// Struct used to update the tasks in the update_task() function passed in to update single points
#[derive(AsChangeset)]
#[diesel(table_name = tasks)]
pub struct UpdatedTask {
    pub title: Option<String>,
    pub description: Option<String>,
    pub completed: Option<bool>,
    pub completed_at: Option<String>,
}
