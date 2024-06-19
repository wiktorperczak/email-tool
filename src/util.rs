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

pub fn parse_year_from_date(header_data: &str) -> Option<u32> {
    let date_prefix = "Date:";
    if let Some(date_index) = header_data.find(date_prefix) {
        let date_start = date_index + date_prefix.len() + 13; // Początek daty po "Date:"
        if let Some(year_str) = header_data.get(date_start..date_start + 4) {
            if let Ok(year) = year_str.trim().parse::<u32>() {
                return Some(year);
            }
        }
    }
    None
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

    #[test]
    fn test_parse_year_from_date_valid() {
        let header_data = "Date: Mon, 21 Jun 2024 15:30:00 +0000";
        let year = parse_year_from_date(header_data);
        assert_eq!(year, Some(2024));
    }

    #[test]
    fn test_parse_year_from_date_no_date_prefix() {
        let header_data = "From: John Doe <john.doe@example.com>";
        let year = parse_year_from_date(header_data);
        assert_eq!(year, None);
    }

    #[test]
    fn test_parse_year_from_date_no_year() {
        let header_data = "Date: Mon, 21 Jun 15:30:00 +0000";
        let year = parse_year_from_date(header_data);
        assert_eq!(year, None);
    }
}
