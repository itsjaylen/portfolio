use std::{ fs::File, io::Read };


use super::{
    args::{ DeveloperCommand, DeveloperError, DeveloperSubcommand },
    modules::tests::Test,
};

#[allow(unused_variables)]
pub fn handle_developer_command(developer: DeveloperCommand) -> Result<(), DeveloperError> {
    let command = developer.command;
    match command {
        DeveloperSubcommand::Test(developer) => {
            if let Err(err) = _test(developer) {
                return Err(DeveloperError::TestError);
            }
        }
        DeveloperSubcommand::TriggerError => {
            _trigger_error()?;
        }
    }

    Ok(())
}

// Commands ran handlers
fn _test(developer: Test) -> Result<(), DeveloperError> {
    println!("Test args: {:?}", developer);
    Ok(())
}

pub fn _trigger_error() -> Result<(), DeveloperError> {
    // Simulating an error condition by attempting to read from a non-existent file
    let file_path = "non_existent_file.txt";
    match File::open(file_path) {
        Ok(mut file) => {
            let mut content = String::new();
            if let Err(_) = file.read_to_string(&mut content) {
                // Returning an Err variant with a specific error type and message
                return Err(DeveloperError::FileError(String::from("Failed to read file content")));
            }
            // If no error, continue with normal operation
            println!("File content: {}", content);
            Ok(())
        }
        Err(_) => {
            // Returning an Err variant with a specific error type and message
            return Err(DeveloperError::FileError(String::from("Failed to open file")));
        }
    }
}


