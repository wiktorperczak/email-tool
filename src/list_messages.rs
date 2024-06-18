use crate::{email_parser, save_to_txt};

extern crate imap;
extern crate native_tls;

use std::net::TcpStream;
use native_tls::TlsStream;
use std::io::{self, BufRead};
use std::time::SystemTime;


pub fn acces_to_messages(mut imap_session : imap::Session<TlsStream<TcpStream>>) -> imap::Session<TlsStream<TcpStream>> {
    println!("Choose one of the options: ");
    println!("1 -- list all messages from last 10 days");
    println!("2 -- list all unread messages");
    println!("3 -- list messages from the given folder");
    println!("4 -- find all messages from given sender");
    
    let mut option = String::new();
    io::stdin().lock().read_line(&mut option).expect("Error loading data");
    let option_number : u32 = option.trim().parse().expect("Incorrect number was given");

    let _ = match option_number {
        1 => emails_last_ten_days(&mut imap_session),
        2 => unread_emails(&mut imap_session),
        3 => emails_from_folder(&mut imap_session),
        4 => search_by_sender(&mut imap_session),
        _ => Ok(()),
    };

    imap_session
}

fn days(number : u64) -> String {
    // 86400 seconds = 1 day
    let ten_days_ago = SystemTime::now() - std::time::Duration::from_secs(86400 * number);

    let date_format = "%d-%b-%Y";
    let date_str = chrono::DateTime::<chrono::Utc>::from(ten_days_ago).format(date_format).to_string();

    date_str
}


fn emails_last_ten_days(imap_session : &mut imap::Session<TlsStream<TcpStream>>) -> imap::error::Result<()> {
    let query = format!("SINCE {}", days(10));
    let uids = imap_session.search(query.as_str())?;

    let mut all_parsed_emails = String::new();
    for uid in uids.iter() {
        let messages = imap_session.fetch(uid.to_string(), "RFC822")?;

        for message in messages.iter() {
            if let Some(body) = message.body() {
                let parsed_content = email_parser::parse_email(body);
                all_parsed_emails.push_str(parsed_content.as_str());
            }
        }
    }
    save_to_txt::save_to_txt(all_parsed_emails);

    Ok(())
}


fn unread_emails(imap_session : &mut imap::Session<TlsStream<TcpStream>>) -> imap::error::Result<()> {
    let query = format!("SINCE {}", days(10));
    let uids = imap_session.search(query)?;

    let mut all_parsed_emails = String::new();
    for uid in uids.iter() {
        let messages = imap_session.fetch(uid.to_string(), "RFC822")?;

        for message in messages.iter() {
            if let Some(body) = message.body() {
                let parsed_content = email_parser::parse_email(body);
                all_parsed_emails.push_str(parsed_content.as_str());
            }
        }
    }
    save_to_txt::save_to_txt(all_parsed_emails);

    Ok(())
}

fn emails_from_folder(imap_session : &mut imap::Session<TlsStream<TcpStream>>) -> imap::error::Result<()> {
    println!("Choose folder:");
    println!("1 -- Starred");
    println!("2 -- Important");
    println!("3 -- Sent Mail");
    println!("4 -- Drafts");
    println!("5 -- Spam");
    println!("6 -- Trash");

    let mut option = String::new();
    io::stdin().lock().read_line(&mut option).expect("Error loading data");
    let option_number : u32 = option.trim().parse().expect("Incorrect number was given");
    
    let folder_prefix = String::from("[Gmail]/");
    let folder_sufix = match option_number {
        1 => String::from("Starred"),
        2 => String::from("Important"),
        3 => String::from("Sent Mail"),
        4 => String::from("Drafts"),
        5 => String::from("Spam"),
        6 => String::from("Trash"),
        _ => {
            println!("Invalid option chosen.");
            String::from("Unknown")
        }
    };
    let folder = format!("{}{}", folder_prefix, folder_sufix);

    imap_session.select(folder).map_err(|e| {
        println!("Error selecting folder\n: {}", e);
        e
    })?;

    let _ = emails_last_ten_days(imap_session);

    Ok(())
}


fn search_by_sender(imap_session : &mut imap::Session<TlsStream<TcpStream>>) -> imap::error::Result<()> {
    println!("Enter email address of a sender: ");
    let mut sender_email = String::new();
    io::stdin().lock().read_line(&mut sender_email).expect("Error loading data");
    let sender_email = sender_email.as_str().trim_end();

    let query = format!("SINCE {}", days(100));
    let uids = imap_session.search(query.as_str())?;

    let mut all_parsed_emails = String::new();
    let mut i : i32 = 0;

    for uid in uids.iter() {
        let messages = imap_session.fetch(uid.to_string(), "RFC822.HEADER")?;

        for message in messages.iter() {
            i += 1;
            if i % 100 == 0 {
                println!("{}", i);
            }
    
            // Pobierz nagłówek "From"
            if let Some(header) = message.header() {
                let header_str = std::str::from_utf8(header).unwrap();              
                if sender_email == get_sender(header_str) {
                    println!("OK");

                    let full_message = imap_session.fetch(uid.to_string(), "RFC822")?;
                    for msg in full_message.iter() {
                        if let Some(body) = msg.body() {
                            let parsed_content = email_parser::parse_email(body);
                            all_parsed_emails.push_str(parsed_content.as_str());
                        }
                    }
                }
            }
        }
    }

    save_to_txt::save_to_txt(all_parsed_emails);
    Ok(())
}

fn get_sender(header_data: &str) -> String {
    for line in header_data.lines() {
        if line.to_lowercase().starts_with("from:") {
            if let Some(start) = line.find('<') {
                if let Some(end) = line.find('>') {
                    return line[start + 1..end].to_string();
                }
            } else {
                // If there are no angle brackets, try to extract the email directly
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
