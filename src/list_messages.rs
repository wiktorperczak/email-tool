use crate::{email_parser, save_to_txt};

extern crate imap;
extern crate native_tls;

use std::net::TcpStream;
use native_tls::TlsStream;
use std::io::{self, BufRead};
use std::time::SystemTime;


pub fn acces_to_messages(imap_session : &mut imap::Session<TlsStream<TcpStream>>) -> imap::error::Result<()> {
    println!("Choose one of the options: ");
    println!("1 -- list last 10 messages");
    
    let mut option = String::new();
    io::stdin().lock().read_line(&mut option).expect("Error loading data");
    let option_number : u32 = option.trim().parse().expect("Incorrect number was given");

    let _ = match option_number {
        1 => emails_last_ten_days(imap_session),
        2 => unread_emails(imap_session),
        _ => Ok(()),
    };

    Ok(())
}

fn emails_last_ten_days(imap_session : &mut imap::Session<TlsStream<TcpStream>>) -> imap::error::Result<()> {
    // Oblicz datę sprzed 10 dni
    let ten_days_ago = SystemTime::now() - std::time::Duration::from_secs(3600 * 1); // 86400 sekund = 1 dzień

    // Formatuj datę do formatu wymaganego przez serwer IMAP (YYYY-MM-DD)
    let date_format = "%d-%b-%Y"; // Format daty zgodny z protokołem IMAP
    let date_str = chrono::DateTime::<chrono::Utc>::from(ten_days_ago).format(date_format).to_string();
    
    // Buduj zapytanie IMAP
    let query = format!("SINCE {}", date_str);

    // Wysyłamy zapytanie do serwera IMAP
    let uids = imap_session.search(query.as_str())?;

    let mut all_parsed_emails = String::new();
    // Iterujemy po każdym UID i pobieramy odpowiednie wiadomości
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

    // Fetch and display the messages
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