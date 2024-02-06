use todo_library::{ database::DatabaseManager, models::UpdatedTask, util };

fn main() {
  loop {
      let mut command_input = String::new();
      print!("Please Enter a command:
      1: create task
      2: delete task
      3: show task
      4: update task
       ");
       let cmd = std::io::stdin().read_line(&mut command_input).unwrap();

       match cmd {
        
       }
  }
    
}

fn nothing() {
    let database_manager = DatabaseManager::new();

    let updated_task = UpdatedTask {
        title: Some("Updated Title".to_string()),
        description: Some("Updated Description".to_string()),
        completed: Some(true),
        completed_at: Some("2022-01-01".to_string()),
    };

    // Update the task with the specified ID using the UpdatedTask struct
    match DatabaseManager::update_task(1, updated_task) {
        Ok(rows_updated) => println!("Updated {} rows", rows_updated),
        Err(err) => eprintln!("Error updating task: {:?}", err),
    }

    println!("{:?}", database_manager.show_task("Updated Title", None))
}
