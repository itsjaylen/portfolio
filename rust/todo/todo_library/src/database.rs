//#TODO Clean this up and document everything.
use std::env;

use diesel::prelude::*;
use dotenvy::dotenv;
use crate::{ models::{ Task, UpdatedTask }, util };

pub fn establish_connection() -> SqliteConnection {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    SqliteConnection::establish(&database_url).unwrap_or_else(|_|
        panic!("Error connecting to {}", database_url)
    )
}

pub struct DatabaseManager {
    pub connection: SqliteConnection,
}

impl DatabaseManager {
    pub fn new() -> Self {
        dotenv().ok();

        let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
        let connection = SqliteConnection::establish(&database_url).unwrap_or_else(|_|
            panic!("Error connecting to {}", database_url)
        );

        Self { connection }
    }

    pub fn create_task(&self, title: &str, description: &str) {
        use crate::schema::tasks;
        use diesel::prelude::*;

        let existing_task = tasks::table
            .filter(tasks::title.eq(title))
            .first::<Task>(&mut establish_connection())
            .ok();

        if let Some(_) = existing_task {
            println!("Task with the same title already exists!");
        } else {
            let new_task = crate::models::NewTask { title, description };

            diesel
                ::insert_into(tasks::table)
                .values(&new_task)
                .execute(&mut establish_connection())
                .expect("Error saving new task");
        }
    }

    pub fn delete_task(&self, task_title: &str) {
        use crate::schema::tasks;
        let pattern = format!("%{}%", task_title);

        let num_deleted = diesel
            ::delete(tasks::table.filter(tasks::title.like(pattern)))
            .execute(&mut establish_connection())
            .expect("Error deleting tasks");

        println!("Deleted {} tasks", num_deleted);
    }

    pub fn show_all_tasks(&self) -> Vec<Task> {
        use crate::schema::tasks::dsl::*;

        tasks
            .select(Task::as_select())
            .load::<Task>(&mut establish_connection())
            .expect("Error loading tasks")
    }

    pub fn show_task(&self, task_title: &str, limit: Option<i16>) -> Option<Task> {
        use crate::schema::tasks::dsl::*;

        let limit = limit.unwrap_or(100);

        let results = tasks
            .filter(title.eq(task_title))
            .limit(limit.into())
            .select(Task::as_select())
            .load::<Task>(&mut establish_connection())
            .expect("Error loading posts");

        if !results.is_empty() {
            for task in &results {
                println!("ID: {}", task.id.unwrap_or_default());
                println!("Title: {}", task.title);
                println!("Description: {:?}", task.description.clone().unwrap_or_default());
                println!(
                    "Created At: {:?}",
                    util::time_util
                        ::convert_time(&task.created_at.clone().unwrap_or_default())
                        .unwrap()
                );
                println!("Completed: {}", task.completed);
                println!("Completed At: {:?}", task.completed_at);
            }

            // Return the first result if any
            results.into_iter().next()
        } else {
            // Handle case when there are no results #TODO Proper logging for errors
            println!("No results found for task '{}'", task_title);
            None
        }
    }

    pub fn update_task(
        task_id: i32,
        updated_task: UpdatedTask
    ) -> Result<usize, diesel::result::Error> {
        use crate::schema::tasks;
        let rows_updated = diesel
            ::update(tasks::table)
            .filter(tasks::id.eq(task_id))
            .set(&updated_task)
            .execute(&mut establish_connection())?;

        Ok(rows_updated)
    }
}
