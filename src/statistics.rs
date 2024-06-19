use crate::util;

extern crate imap;
extern crate native_tls;

use std::net::TcpStream;
use native_tls::TlsStream;
use chrono::{Datelike, Utc};
use std::collections::HashMap;


pub fn generate_statistics(mut imap_session : imap::Session<TlsStream<TcpStream>>) -> imap::Session<TlsStream<TcpStream>> {
    let line = "_".repeat(50);
    println!("\n{}\n", line);

    let _ = folder_statistics(&mut imap_session);
    let _ = stats_by_year(&mut imap_session);
    let _ = stats_sender(&mut imap_session);
    
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
                
                if let Some(year) = util::parse_year_from_date(header_str) {
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

fn stats_sender(imap_session : &mut imap::Session<TlsStream<TcpStream>>) -> imap::error::Result<()> {
    let mut email_count_map: HashMap<String, usize> = HashMap::new();
    
    imap_session.select("INBOX")?;
    let query = format!("SINCE {}", util::days(15));
    let uids = imap_session.search(query.as_str())?;

    for uid in uids.iter() {
        let messages = imap_session.fetch(uid.to_string(), "RFC822.HEADER")?;

        for message in messages.iter() {
            if let Some(header) = message.header() {
                let header_str = util::get_sender(std::str::from_utf8(header).unwrap());
                *email_count_map.entry(header_str).or_insert(0) += 1;
            }
        }
    }

    let mut pairs: Vec<_> = email_count_map.into_iter().collect();

    pairs.sort_by_key(|&(_, v)| std::cmp::Reverse(v));

    println!("\nMost messages sent (last 30 days): ");
    for (i, (key, value)) in pairs.iter().take(10).enumerate() {
        println!("{}. {}: {}", i + 1, key, 2 * value);
    }

    Ok(())
}

