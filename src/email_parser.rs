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
    let combined_content = format!("From:{}\nDate:{}\n\n{}\n\n{}\n\n", from, date, text_body, line);
    
    combined_content
}