use crate::{email_parser, save_to_txt};

extern crate imap;
extern crate native_tls;

use std::net::TcpStream;
use native_tls::TlsStream;
use std::io::{self, BufRead};
use std::time::SystemTime;


pub fn acces_to_messages(imap_session : &mut imap::Session<TlsStream<TcpStream>>) -> imap::error::Result<()> {
    println!("Choose one of the options: ");
    println!("1 -- list all messages from last 10 days");
    println!("2 -- list all unread messages");
    println!("3 -- list messages (from last 10 days) from the given folder");
    println!("4 -- find all messages from given sender");
    println!("5 -- find all messages with the given key word");
    
    let mut option = String::new();
    io::stdin().lock().read_line(&mut option).expect("Error loading data");
    let option_number : u32 = option.trim().parse().expect("Incorrect number was given");

    let _ = match option_number {
        1 => emails_last_ten_days(imap_session),
        2 => unread_emails(imap_session),
        3 => emails_from_folder(imap_session),
        _ => Ok(()),
    };

    Ok(())
}


fn emails_last_ten_days(imap_session : &mut imap::Session<TlsStream<TcpStream>>) -> imap::error::Result<()> {
    // 86400 seconds = 1 day
    let ten_days_ago = SystemTime::now() - std::time::Duration::from_secs(86400 * 10);

    let date_format = "%d-%b-%Y"; // Format daty zgodny z protokołem IMAP
    let date_str = chrono::DateTime::<chrono::Utc>::from(ten_days_ago).format(date_format).to_string();

    let query = format!("SINCE {}", date_str);
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
    let query = "NEW";
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
        println!("Error selecting INBOX\n: {}", e);
        e
    })?;

    let _ = emails_last_ten_days(imap_session);

    Ok(())
}

// fn search_by_sender() {

// }

// fn search_by_key_word() {

// }