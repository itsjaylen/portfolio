//#TODO Clean this up and document everything.
use std::env;

use diesel::prelude::*;
use dotenvy::dotenv;

use crate::{ models::Task, util };

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

/// Implementation of DatabaseManager functionality.
impl DatabaseManager {
    /// Creates a new DatabaseManager instance.
    ///
    /// # Returns
    ///
    /// Returns a new instance of DatabaseManager.
    ///
    /// # Example
    ///
    /// ```
    /// let database_manager = DatabaseManager::new();
    /// ```
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

        let new_task = crate::models::NewTask { title, description };

        diesel
            ::insert_into(tasks::table)
            .values(&new_task)
            .execute(&mut establish_connection())
            .expect("Error saving new task");
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

    /// Shows information about a task with the given title.
    ///
    /// This function retrieves information about a task with the specified title from the database
    /// and prints the details. If the task is found, it returns the information as an `Option<Task>`.
    /// If the task is not found, it prints an error message and returns `None`.
    ///
    /// # Arguments
    ///
    /// * `task_title` - The title of the task to be shown.
    /// * `limit` - An optional limit on the number of tasks to retrieve.
    ///
    /// # Returns
    ///
    /// Returns an `Option<Task>` containing information about the task if found; otherwise, returns `None`.
    ///
    /// # Example
    ///
    /// ```
    /// let database_manager = DatabaseManager::new();
    /// let task_info = database_manager.show_task("Example Task", Some(2));
    /// if let Some(task) = task_info {
    ///     // Do something with the task information
    /// } else {
    ///     // Handle the case when the task is not found
    /// }
    /// ``
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
}
