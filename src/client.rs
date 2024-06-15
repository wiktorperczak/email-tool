extern crate imap;
extern crate native_tls;

use std::net::TcpStream;
use native_tls::TlsStream;
use imap::{Client, error};
use std::io::{self, BufRead, Write};
use rpassword::read_password;

use crate::list_messages;

fn get_login_data() -> (String, String) {
    // rust1.project2@zohomail.eu
    // Rust1!.Project2@

    let mut email = String::new();
    println!("Enter your e-mail: ");
    io::stdin().lock().read_line(&mut email).expect("Error loading data");

    println!("Enter your password: ");
    io::stdout().flush().unwrap();
    let password = read_password().unwrap();
    // io::stdin().lock().read_line(&mut password).expect("Error loading data");

    (email.trim().to_string(), password.trim().to_string())
}


pub fn run() -> error::Result<()> {
    println!("Welcome to the email manager app!");

    let domain = "imap.zoho.eu";
    let tls = native_tls::TlsConnector::builder().build().unwrap();
    let client: Client<TlsStream<TcpStream>> = imap::connect((domain, 993), domain, &tls).unwrap();

    let (email, password) = get_login_data();

    let mut imap_session = client.login(email, password).map_err(|e| e.0)?;
    imap_session.select("INBOX")?;

    println!("You have logged in correctly.");

    loop {
        println!("Choose one of the options: ");
        let mut option = String::new();
        io::stdin().lock().read_line(&mut option).expect("Error loading data");
        let option_number : u32 = option.trim().parse().expect("Incorrect number was given");

        let imap_session_ref = &mut imap_session;
        
        let _ = match option_number {
            1 => list_messages::acces_to_messages(imap_session_ref),
            2 => { println!("Statistics"); Ok(()) },
            _ => Ok(()), 
        };

        if option_number == 3 {
            break;
        }
    }

    Ok(())
}