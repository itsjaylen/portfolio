use crate::db::database::DatabaseManager;
mod db;
mod models;
mod schema;
mod util;


fn main() {
    let database_manager = DatabaseManager::new();
    //database_manager.create_task("Database Task1", "My description");
    //database_manager.delete_task("Database Task1")
    //database_manager.show_all_tasks();
    database_manager.show_task("Test Task", None);

}


