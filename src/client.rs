extern crate imap;
extern crate native_tls;
use crate::list_messages;
use crate::statistics;

use std::net::TcpStream;
use imap::Authenticator;
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
        println!("1 -- Show messages");
        println!("2 -- Get statistics");
        println!("3 -- Finish session");

        let mut option = String::new();
        io::stdin().lock().read_line(&mut option).expect("Error loading data");
        let option_number : u32 = option.trim().parse().expect("Incorrect data was given");
        
        imap_session = match option_number {
            1 => list_messages::acces_to_messages(imap_session),
            2 => statistics::generate_statistics(imap_session),
            _ => imap_session, 
        };

        if option_number == 3 {
            break;
        }
    }

    Ok(())
}



struct MockAuthenticator;

impl Authenticator for MockAuthenticator {
    type Response = &'static str; // Określenie typu odpowiedzi

    fn process(&self, _data: &[u8]) -> Self::Response {
        "mock_response"
    }
}

pub trait ImapClient {
    fn authenticate(&self, authenticator: Box<dyn Authenticator<Response = String>>) -> Result<(), String>;
    fn select(&mut self, folder: &str) -> Result<(), String>;
    fn fetch(&mut self, sequence_set: &str, query: &str) -> Result<Vec<String>, String>;
}

pub struct RealImapClient {
    // Tutaj mogą być przechowywane informacje potrzebne do połączenia z serwerem IMAP
}

impl ImapClient for RealImapClient {
    fn authenticate(&self, _authenticator: Box<dyn Authenticator<Response = String>>) -> Result<(), String> {
        println!("Real client: Authenticating...");
        // Symulacja rzeczywistego procesu autentykacji
        Ok(())
    }

    fn select(&mut self, folder: &str) -> Result<(), String> {
        println!("Real client: Selecting folder {}...", folder);
        // Symulacja rzeczywistego procesu wyboru folderu
        Ok(())
    }

    fn fetch(&mut self, sequence_set: &str, query: &str) -> Result<Vec<String>, String> {
        println!("Real client: Fetching messages with sequence set {} and query {}...", sequence_set, query);
        // Symulacja rzeczywistego procesu pobierania wiadomości
        Ok(vec![
            "Message 1".to_string(),
            "Message 2".to_string(),
            "Message 3".to_string(),
        ])
    }
}

pub struct MockImapClient {
    // Można dodać stan, jeśli to konieczne do testów
}

impl ImapClient for MockImapClient {
    fn authenticate(&self, _authenticator: Box<dyn Authenticator<Response = String>>) -> Result<(), String> {
        println!("Mock client: Authenticating...");
        // Symulacja prostego mocka autentykacji
        Ok(())
    }

    fn select(&mut self, folder: &str) -> Result<(), String> {
        println!("Mock client: Selecting folder {}...", folder);
        // Symulacja prostego mocka wyboru folderu
        Ok(())
    }

    fn fetch(&mut self, sequence_set: &str, query: &str) -> Result<Vec<String>, String> {
        println!("Mock client: Fetching messages with sequence set {} and query {}...", sequence_set, query);
        // Symulacja prostego mocka pobierania wiadomości
        Ok(vec![
            "Mock Message 1".to_string(),
            "Mock Message 2".to_string(),
            "Mock Message 3".to_string(),
        ])
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    struct MockAuthenticator;

    impl Authenticator for MockAuthenticator {
        type Response = String;

        fn process(&self, _data: &[u8]) -> Self::Response {
            "mock_token".to_string()
        }
    }

    #[test]
    fn test_mock_imap_client_operations() {
        let mut client = MockImapClient {};

        let result = client.authenticate(Box::new(MockAuthenticator {}));
        assert!(result.is_ok());

        let result = client.select("INBOX");
        assert!(result.is_ok());

        let result = client.fetch("1:3", "ALL");
        assert!(result.is_ok());
        let messages = result.unwrap();
        assert_eq!(messages.len(), 3);
        assert_eq!(messages[0], "Mock Message 1");
        assert_eq!(messages[1], "Mock Message 2");
        assert_eq!(messages[2], "Mock Message 3");
    }
}