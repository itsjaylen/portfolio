use std::io;

use todo_library::{ database::DatabaseManager, models::UpdatedTask };

#[derive(Debug)]
struct Task {
    title: String,
    description: String,
}

fn get_input(prompt: &str, input_buffer: &mut String) {
    println!("{}", prompt);
    io::stdin().read_line(input_buffer).unwrap();
    *input_buffer = input_buffer.trim().to_string();
}

impl Task {
    fn create() -> Task {
        let mut task_title = String::new();
        let mut task_description = String::new();

        get_input("Please enter task name...", &mut task_title);
        get_input("Please enter task description...", &mut task_description);

        Task {
            title: task_title.trim().to_string(),
            description: task_description.trim().to_string(),
        }
    }

    fn add_tasks(task_list: &[Task], database_manager: &DatabaseManager) {
        task_list.iter().for_each(|task| {
            database_manager.create_task(&task.title.trim(), &task.description.trim());
        });
    }

    fn delete_task(database_manager: &DatabaseManager) {
        let mut task_title = String::new();

        get_input("Please enter the task name", &mut task_title);
        database_manager.delete_task(&task_title.trim());
    }

    fn show_task(database_manager: &DatabaseManager) {
        let mut task_title = String::new();

        get_input("Please enter the task name", &mut task_title);
        database_manager.show_task(&task_title.trim(), Some(10));
    }

    fn show_all_tasks(database_manager: &DatabaseManager) {
        let all_tasks = database_manager.show_all_tasks();
        all_tasks.iter().for_each(|task| {
            println!("{:?}", task);
        });
    }

    fn update_task() {
        // TODO: Implement update_task
    }
}

fn main() {
    let database_manager = DatabaseManager::new();
    database_manager.delete_task("task_title");

    loop {
        let mut command_input = String::new();

        println!(
            "Please Enter a command:
            1: create task
            2: delete task
            3: show task
            4: show all tasks
            5: update task
            0: exit"
        );

        print!("Enter your choice: ");
        io::stdin().read_line(&mut command_input).unwrap();

        match command_input.trim().parse() {
            Ok(cmd) =>
                match cmd {
                    1 => {
                        let task_list = Task::create();
                        Task::add_tasks(&[task_list], &database_manager);
                    }
                    2 => Task::delete_task(&database_manager),
                    3 => Task::show_task(&database_manager),
                    4 => Task::show_all_tasks(&database_manager),
                    5 => Task::update_task(),
                    0 => {
                        println!("Exiting the program. Goodbye!");
                        return;
                    }
                    _ => println!("Invalid command. Please enter a valid option."),
                }
            Err(_) => println!("Invalid input. Please enter a number."),
        }
    }
}
