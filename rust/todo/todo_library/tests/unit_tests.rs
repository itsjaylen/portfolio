#[cfg(test)]
mod tests {
    use todo_library::database::DatabaseManager;
    

  
    #[test]
    fn test_create_task() {
        let database_manager = DatabaseManager::new();
        database_manager.create_task("Test Task", "Test Description");
        let created_task = database_manager.show_task("Test Task", None);
        assert!(created_task.is_some(), "Task should be created.");
    }

    #[test]
    fn test_delete_task() {
        let database_manager = DatabaseManager::new();
        database_manager.delete_task("Test Task");
        let deleted_task = database_manager.show_task("Test Task", None);
        assert!(deleted_task.is_none(), "Task should be deleted.");
    }

    #[test]
    fn test_show_all_tasks() {
        let database_manager = DatabaseManager::new();
        let all_shown_tasks = database_manager.show_all_tasks();
        assert!(!all_shown_tasks.is_empty(), "No tasks were shown");
    }

    #[test]
    fn test_show_task() {
        let database_manager = DatabaseManager::new();
        let shown_task = database_manager.show_task("Test Task", None);
        assert!(!shown_task.is_some(), "No tasks were shown");
    }
}