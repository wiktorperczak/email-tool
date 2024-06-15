use crate::{email_parser, save_to_txt};

extern crate imap;
extern crate native_tls;

use std::net::TcpStream;
use native_tls::TlsStream;
use std::io::{self, BufRead};


pub fn acces_to_messages(imap_session : &mut imap::Session<TlsStream<TcpStream>>) -> imap::error::Result<()> {
    println!("Choose one of the options: ");
    println!("1 -- list last 10 messages");
    
    let mut option = String::new();
    io::stdin().lock().read_line(&mut option).expect("Error loading data");
    let option_number : u32 = option.trim().parse().expect("Incorrect number was given");

    let _ = match option_number {
        1 => unread_emails(imap_session),
        _ => Ok(()),
    };

    Ok(())
}


fn unread_emails(imap_session : &mut imap::Session<TlsStream<TcpStream>>) -> imap::error::Result<()> {
    let query = "NEW";
    let uids = imap_session.search(query)?;

    // Fetch and display the messages
    for uid in uids.iter() {
        let messages = imap_session.fetch(uid.to_string(), "RFC822")?;
        let mut all_parsed_emails = String::new();

        for message in messages.iter() {
            if let Some(body) = message.body() {
                let parsed_content = email_parser::parse_email(body);
                all_parsed_emails.push_str(parsed_content.as_str());
            }
        }
        save_to_txt::save_to_txt(all_parsed_emails);
    }

    Ok(())
}