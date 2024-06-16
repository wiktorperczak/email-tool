extern crate imap;
extern crate native_tls;
use crate::list_messages;

use std::net::TcpStream;
use native_tls::TlsStream;
use imap::{Client, error};
use std::io::{self, BufRead, Write};
use rpassword::read_password;


struct GmailOAuth2 {
    user: String,
    access_token: String,
}

impl imap::Authenticator for GmailOAuth2 {
    type Response = String;
    #[allow(unused_variables)]
    fn process(&self, data: &[u8]) -> Self::Response {
        format!(
            "user={}\x01auth=Bearer {}\x01\x01",
            self.user, self.access_token
        )
    }
}


fn get_login_data() -> (String, String) {
    // rust1.project2@zohomail.eu
    // Rust1!.Project2@

    let mut email = String::new();
    println!("Enter your e-mail: ");
    io::stdin().lock().read_line(&mut email).expect("Error loading data");

    println!("Enter your access token: ");
    io::stdout().flush().unwrap();
    let access_token = read_password().unwrap();
    // io::stdin().lock().read_line(&mut password).expect("Error loading data");

    (email.trim().to_string(), access_token.trim().to_string())
}


pub fn run() -> error::Result<()> {
    println!("Welcome to the email manager app!");

    let domain = "imap.gmail.com";
    let tls = native_tls::TlsConnector::builder().build().unwrap();
    let client: Client<TlsStream<TcpStream>> = imap::connect((domain, 993), domain, &tls).unwrap();

    let (user_email, access_token) = get_login_data();
    let gmail_auth = GmailOAuth2 {
        user: user_email,
        access_token: access_token,
    };

    let mut imap_session = client.authenticate("XOAUTH2", &gmail_auth).map_err(|e| {
        eprintln!("Login error!");
        e.0
    })?;

    println!("You have logged in correctly.");

    imap_session.select("INBOX").map_err(|e| {
        println!("Error selecting INBOX\n: {}", e);
        e
    })?;

    loop {
        println!("\nChoose one of the options: ");
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