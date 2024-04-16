use std::fs::File;
use std::io::{self, BufRead, BufReader};

use clap::Args;


#[derive(Debug, Args)]
pub struct ShowLogs {
    /// The amount of logs to show
    pub log_amount: i64,
}

//TODO PHARSE THEM
pub fn check_server_logs(log_limit: i64) -> io::Result<()> {
    // Specify the path to the log file
    let file_path = "/tmp/icecli.log";
    
    // Open the log file for reading
    let file = File::open(file_path)?;
    
    // Create a buffered reader
    let reader = BufReader::new(file);
    
    // Initialize a counter to track the number of lines read
    let mut line_count = 0;
    
    // Iterate through each line in the file
    for line in reader.lines() {
        // Stop reading if the log limit is reached
        if line_count >= log_limit {
            break;
        }
        
        // Read the line and handle any errors
        match line {
            Ok(line) => {
                // Print the line or process it as desired
                println!("{}", line);
                
                // Increment the line count
                line_count += 1;
            }
            Err(e) => {
                // Handle the error (e.g., print an error message)
                eprintln!("Error reading line: {}", e);
            }
        }
    }
    
    Ok(())
}
