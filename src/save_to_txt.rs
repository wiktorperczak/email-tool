use std::fs::{OpenOptions, create_dir_all};
use std::io::Write;
use std::path::Path;
use chrono::Local;

fn append_to_file(file_path: &str, text: &str) -> std::io::Result<()> {
    // CHecks if folder exists, if not creates the directory
    if let Some(parent) = Path::new(file_path).parent() {
        create_dir_all(parent)?;
    }

    // Opens the file in append mode, creates the file if it does not exist
    let mut file = OpenOptions::new()
        .write(true)
        .append(true)
        .create(true)
        .open(file_path)?;

    // Append text to the file
    writeln!(file, "{}", text)?;
    Ok(())
}

pub fn save_to_txt(email : String) {
    // Get current time and date
    let now = Local::now();
    // Format date and time to string
    let formatted_date = now.format("%Y-%m-%d_%H-%M-%S").to_string();

    // create path to the file
    let file_name = format!("emails/{}.txt", formatted_date);

    match append_to_file(&file_name, email.as_str()) {
        Ok(_) => println!("Messages successfully saved to the .txt file."),
        Err(e) => eprintln!("Failed to save the messages: {}", e),
    }
}
