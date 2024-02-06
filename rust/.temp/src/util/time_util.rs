use chrono::{Local, NaiveDateTime, TimeZone};

pub fn convert_time(timestamp_str: &str) -> Option<String> {
  // Attempt to parse the timestamp string into a NaiveDateTime object
  match NaiveDateTime::parse_from_str(timestamp_str, "%Y-%m-%d %H:%M:%S") {
      Ok(naive_timestamp) => {
          // Convert the NaiveDateTime to the local time
          let local_time = Local.from_local_datetime(&naive_timestamp).earliest();

          // Check if the conversion to local time was successful
          if let Some(local_time) = local_time {
              // Format the time with day of the week, year, and 12-hour clock with AM/PM
              let formatted_time = local_time.format("%A %Y-%m-%d %I:%M:%S %p").to_string();

              // Return the formatted time
              Some(formatted_time)
          } else {
              eprintln!("Error converting to local time");
              // Handle the conversion error as needed
              None
          }
      }
      Err(err) => {
          eprintln!("Error parsing timestamp: {}", err);
          // Handle the parsing error as needed
          None
      }
  }
}