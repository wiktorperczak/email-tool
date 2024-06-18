use std::fs::{OpenOptions, create_dir_all};
use std::io::Write;
use std::path::Path;
use chrono::Local;

fn append_to_file(file_path: &str, text: &str) -> std::io::Result<()> {
    // Checks if folder exists, if not creates the directory
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


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_append_to_file() {
        // Test case 1: Successful append to file
        let file_path = "test_append_to_file.txt";
        let text = "Test message 1";
        assert!(append_to_file(file_path, text).is_ok());

        // Test case 2: Append to existing file
        let text = "Test message 2";
        assert!(append_to_file(file_path, text).is_ok());

        // Clean up test file after test cases
        std::fs::remove_file(file_path).unwrap();
    }

    #[test]
    fn test_save_to_txt() {
        // Test case 1: Save email to txt file
        let email = "Sample email content".to_string();
        save_to_txt(email.clone());
        let now = Local::now();
        let formatted_date = now.format("%Y-%m-%d_%H-%M-%S").to_string();
        let file_path = format!("emails/{}.txt", formatted_date);
        let file_content = std::fs::read_to_string(&file_path).unwrap();
        assert!(file_content.contains(&email));

        // Clean up test file after test case
        std::fs::remove_file(file_path).unwrap();
    }
}