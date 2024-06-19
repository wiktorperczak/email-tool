use std::time::SystemTime;

pub fn days(number : u64) -> String {
    // 86400 seconds = 1 day
    let ten_days_ago = SystemTime::now() - std::time::Duration::from_secs(86400 * number);

    let date_format = "%d-%b-%Y";
    let date_str = chrono::DateTime::<chrono::Utc>::from(ten_days_ago).format(date_format).to_string();

    date_str
}

pub fn get_sender(header_data: &str) -> String {
    for line in header_data.lines() {
        if line.to_lowercase().starts_with("from:") {
            if let Some(start) = line.find('<') {
                if let Some(end) = line.find('>') {
                    return line[start + 1..end].to_string();
                }
            } else {
                let parts: Vec<&str> = line.split_whitespace().collect();
                for part in parts {
                    if part.contains('@') {
                        return part.to_string();
                    }
                }
            }
        }
    }
    "Unknown".to_string()
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_sender_with_angle_brackets() {
        let header_data = "From: John Doe <john.doe@example.com>";
        let sender = get_sender(header_data);
        assert_eq!(sender, "john.doe@example.com");
    }

    #[test]
    fn test_get_sender_without_angle_brackets() {
        let header_data = "From: jane.doe@example.com";
        let sender = get_sender(header_data);
        assert_eq!(sender, "jane.doe@example.com");
    }

    #[test]
    fn test_get_sender_with_mixed_format() {
        let header_data = "From: John Doe <john.doe@example.com>";
        let sender = get_sender(header_data);
        assert_eq!(sender, "john.doe@example.com");

        let header_data = "From: jane.doe@example.com";
        let sender = get_sender(header_data);
        assert_eq!(sender, "jane.doe@example.com");
    }

    #[test]
    fn test_get_sender_case_insensitivity() {
        let header_data = "FROM: John Doe <john.doe@example.com>";
        let sender = get_sender(header_data);
        assert_eq!(sender, "john.doe@example.com");

        let header_data = "from: jane.doe@example.com";
        let sender = get_sender(header_data);
        assert_eq!(sender, "jane.doe@example.com");
    }

    #[test]
    fn test_get_sender_unknown_format() {
        let header_data = "Subject: Hello World";
        let sender = get_sender(header_data);
        assert_eq!(sender, "Unknown");
    }
}
