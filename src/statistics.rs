extern crate imap;
extern crate native_tls;

use std::net::TcpStream;
use native_tls::TlsStream;
use chrono::{Datelike, Utc};

pub fn generate_statistics(mut imap_session : imap::Session<TlsStream<TcpStream>>) -> imap::Session<TlsStream<TcpStream>> {
    let line = "_".repeat(50);
    println!("\n{}\n", line);

    let _ = folder_statistics(&mut imap_session);
    let _ = stats_by_year(&mut imap_session);
    
    println!("\n{}\n", line);
    
    imap_session
}

fn stats_by_year(imap_session : &mut imap::Session<TlsStream<TcpStream>>) -> imap::error::Result<()> {
    imap_session.select("INBOX")?;

    let uids = imap_session.search("1")?;
    let mut first_year : u32 = 2010;


    for uid in uids.iter() {
        let messages = imap_session.fetch(uid.to_string(), "RFC822.HEADER")?;
    
        for message in messages.iter() {
            if let Some(header) = message.header() {
                let header_str = std::str::from_utf8(header).unwrap();    
                
                if let Some(year) = parse_year_from_date(header_str) {
                    first_year = year;
                }
            }
        }
    
    }

    let current_year = Utc::now().year();
    let first_year = first_year as i32;

    for year in (first_year..=current_year).rev() {
        let start_date_str = format!("01-Jan-{}", year);
        let end_date_str = format!("01-Jan-{}", year+1);

        let search_criteria = format!("SINCE {} BEFORE {}", start_date_str, end_date_str);
        let uids = imap_session.search(&search_criteria)?;

        println!("Number of messages in {}: {}", year, uids.len());
    }

    Ok(())
}



fn folder_statistics(imap_session : &mut imap::Session<TlsStream<TcpStream>>) -> imap::error::Result<()> {
    let folders = ["Starred", "Important",
               "Sent Mail", "Spam", "Trash"];
               
               let uids = imap_session.search("ALL")?;
    println!("Total number of messages: {}\n", uids.len());

    for folder in folders {
        imap_session.select(format!("{}{}", "[Gmail]/", folder))?;
        let uids = imap_session.search("ALL")?;
        println!("Number of messages in {}: {}", folder, uids.len());
    }
    
    println!();
    Ok(())
}


fn parse_year_from_date(header_data: &str) -> Option<u32> {
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