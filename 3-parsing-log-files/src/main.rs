use flate2::read::GzDecoder;
use regex::Regex;
use std::env;
use std::fs::File;

/// Reads a log file, decompresses it if necessary, and calculates the error rate/
/// Note, this function assumes that the log file contains timestamps in the format "YYYY-MM-DD HH:MM:SS-ZZ"
/// and that the log entries contain the keyword "Error". It calculates the error rate for each hour in the log file.
/// This function prints the error count for each hour and the total error count for the log file.
/// The function uses the `flate2` crate to decompress gzipped files and the `regex` crate to match timestamps in the log file.
/// The function reads the log file line by line, extracts the timestamp from each line, and calculates the error rate for each hour.
/// The function should be adapted to match the specific format of the log file and the particular keyword.
///
/// # Arguments
///
/// * `file_path` - A string slice that holds the path to the log file
///
/// # Example
/// To locate the log files, compress/uncompress them, and read their contents, run the following commands as needed:
/// find ../logs -name "*.txt"
/// find ../logs -name "*.gz"
/// gzip ../logs/dummy-log.txt
/// unzip ../logs/logs.zip //these are zipped log files from GitHub codespaces
/// cat ../logs/dummy-log.txt
/// cargo run ../logs/dummy-log.txt
/// cargo run ../logs/dummy-log.gz

fn read_buffer(file_path: &str) {
    // Initialise variables for error rate calculation
    let mut total_entries = 0;
    let mut error_entries = 0;
    let mut current_hour = None;

    // Regular expression to match timestamps in the log file
    // The regex pattern `(\d{4}-\d{2}-\d{2} \d{2}:\d{2}:\d{2}-\d{2})` matches "YYYY-MM-DD HH:MM:SS-ZZ":
    // - `\d{4}`: exactly 4 digits (year)
    // - `-`: a hyphen
    // - `\d{2}`: exactly 2 digits (month)
    // - `-`: a hyphen
    // - `\d{2}`: exactly 2 digits (day)
    // - ` `: a space
    // - `\d{2}`: exactly 2 digits (hour)
    // - `:`: a colon
    // - `\d{2}`: exactly 2 digits (minute)
    // - `:`: a colon
    // - `\d{2}`: exactly 2 digits (second)
    // - `-`: a hyphen
    // - `\d{2}`: exactly 2 digits (timezone offset)
    let timestamp_regex = Regex::new(r"(\d{4}-\d{2}-\d{2} \d{2}:\d{2}:\d{2}-\d{2})").unwrap();
    let error_keyword = "Error"; // Keyword to search for in log entries

    // Open the log file.
    let file = File::open(file_path).unwrap();
    use std::io::{BufRead, BufReader};

    // Create a buffered reader, decompressing if the file is gzipped
    let reader: Box<dyn BufRead> = match file_path.ends_with(".gz") {
        true => {
            // Decompress gzipped file.
            let decompressor = GzDecoder::new(file);
            Box::new(BufReader::new(decompressor))
        }
        false => Box::new(BufReader::new(file)),
    };

    // Read the log file line by line
    for line in reader.lines() {
        let line = match line {
            Ok(line) => line,
            Err(error) => {
                eprintln!("Error reading line: {}", error);
                continue;
            }
        };

        // Extract timestamp from the log line
        if let Some(captures) = timestamp_regex.captures(&line) {
            let timestamp = &captures[1];
            // Extract the date and hour part of the timestamp, assuming it's in "YYYY-MM-DD HH:MM:SS-ZZ" format
            let date = &timestamp[0..10];
            let hour = &timestamp[11..13];
            let date_hour = format!("{}, Hour: {}", date, hour);

            // Update the current hour and reset counters if necessary
            if current_hour != Some(date_hour.to_string()) {
                // Calculate and print error count for the previous hour
                if let Some(prev_hour) = current_hour.take() {
                    println!("{prev_hour} - Error Count: {error_entries}");
                }

                // Reset counters for the new hour
                error_entries = 0;
                current_hour = Some(date_hour.to_string());
            }

            // Check if the log entry contains an error
            if line.contains(error_keyword) {
                error_entries += 1;
                total_entries += 1;
            }
        }
    }

    println!("Total error count for current log: {total_entries}");
}
fn main() {
    // Collect command-line arguments
    let args: Vec<String> = env::args().collect();

    // Check if the user provided a path argument
    if args.len() < 2 {
        eprintln!("Usage: log_error_rate <log_file_path>");
        std::process::exit(1);
    }

    let log_file_path = &args[1];

    read_buffer(log_file_path);
}
