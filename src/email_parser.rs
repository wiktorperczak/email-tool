use mailparse::{parse_mail, MailHeaderMap};


pub fn parse_email(body : &[u8]) -> String {
    let parsed_message = parse_mail(body).unwrap();

    // Function to recursively extract text from the email parts
    fn get_text_part(parsed: &mailparse::ParsedMail) -> String {
        if parsed.subparts.is_empty() {
            if let Ok(text) = parsed.get_body() {
                return text;
            }
        } else {
            for subpart in &parsed.subparts {
                let text = get_text_part(subpart);
                if !text.is_empty() {
                    return text;
                }
            }
        }
        String::new()
    }

    let text_body = get_text_part(&parsed_message);

    // Get the sender and date from the headers
    let from = parsed_message.headers.get_first_value("From").unwrap_or_else(|| "Unknown".to_string());
    let date = parsed_message.headers.get_first_value("Date").unwrap_or_else(|| "Unknown".to_string());

    let line = "_".repeat(50);
    let combined_content = format!("From:{}\nDate:{}\n\n{}\n{}\n\n\n", from, date, text_body, line);

    combined_content
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_email_with_plain_text() {
        let email = b"\
                    From: sender@example.com\r\n\
                    Date: Tue, 15 Jun 2021 16:02:00 +0200\r\n\
                    Content-Type: text/plain; charset=\"UTF-8\"\r\n\
                    \r\n\
                    This is a plain text email body.";
                            
        let result = parse_email(email);
        let expected = "From:sender@example.com\nDate:Tue, 15 Jun 2021 16:02:00 +0200\n\nThis is a plain text email body.\n__________________________________________________\n\n\n";
        assert_eq!(result, expected);
    }

    #[test]
    fn test_parse_email_with_unknown_fields() {
        let email = b"\
From: unknown@example.com\r\n\
Content-Type: text/plain; charset=\"UTF-8\"\r\n\
\r\n\
This email has an unknown date.";
        
        let result = parse_email(email);
        let expected = "From:unknown@example.com\nDate:Unknown\n\nThis email has an unknown date.\n__________________________________________________\n\n\n";
        assert_eq!(result, expected);
    }

    #[test]
    fn test_parse_email_with_no_body() {
        let email = b"\
From: sender@example.com\r\n\
Date: Tue, 15 Jun 2021 16:02:00 +0200\r\n\
Content-Type: text/plain; charset=\"UTF-8\"\r\n\
\r\n";
        
        let result = parse_email(email);
        let expected = "From:sender@example.com\nDate:Tue, 15 Jun 2021 16:02:00 +0200\n\n\n__________________________________________________\n\n\n";
        assert_eq!(result, expected);
    }
}