extern crate imap;
extern crate native_tls;
// use mailparse::parse_mail;
use mailparse::{parse_mail, MailHeaderMap};

fn main() {
    // To connect to the gmail IMAP server with this you will need to allow unsecure apps access.
    // See: https://support.google.com/accounts/answer/6010255?hl=en
    // Look at the gmail_oauth2.rs example on how to connect to a gmail server securely.
    fetch_inbox_top().unwrap();
}

fn fetch_inbox_top() -> imap::error::Result<()> {
    let domain = "imap.zoho.eu";
    let tls = native_tls::TlsConnector::builder().build().unwrap();

    // we pass in the domain twice to check that the server's TLS
    // certificate is valid for the domain we're connecting to.
    let client = imap::connect((domain, 993), domain, &tls).unwrap();

    // the client we have here is unauthenticated.
    // to do anything useful with the e-mails, we need to log in
    let mut imap_session = client
        .login("rust1.project2@zohomail.eu", "Rust1!.Project2@")
        .map_err(|e| e.0)?;

    // we want to fetch the first email in the INBOX mailbox
    imap_session.select("INBOX")?;

    // fetch message number 1 in this mailbox, along with its RFC822 field.
    // RFC 822 dictates the format of the body of e-mails
    
    // let messages = imap_session.fetch("4", "RFC822")?;
    // let message = if let Some(m) = messages.iter().next() {
    //     m
    // } else {
    //     return Ok(None);
    // };

    // // extract the message's body
    // let body = message.body().expect("message did not have a body!");
    // let body = std::str::from_utf8(body)
    //     .expect("message was not valid utf-8")
    //     .to_string();
    // println!("{}", body);

    // Oblicz datę sprzed 30 dni
    // let thirty_days_ago = Utc::now() - Duration::days(30);
    // let search_date = thirty_days_ago.format("%d-%b-%Y").to_string();

    // Wyszukiwanie wiadomości od tej daty
    // let query = format!("SINCE {}", search_date);
    // let uids = imap_session.search(query)?;

    println!("RECIEVING EMAILS STARTED!!!");
    println!();

    let query = "OLD";
    let uids = imap_session.search(query)?;

    // Fetch and display the messages
    for uid in uids.iter() {
        let messages = imap_session.fetch(uid.to_string(), "RFC822")?;
        for message in messages.iter() {
            if let Some(body) = message.body() {
                let body_str = std::str::from_utf8(body).expect("message was not valid utf-8");
                println!("{}", body_str);

                println!();
                println!();
                println!();
                println!("NOW PARSED VERSION!");


                //TODO: przerobić przerobić parsowanie na osobną funkcję
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

                println!("From: {}", from);
                println!("Date: {}", date);
                println!("Body: {}", text_body);
            }
        }
    }


    // be nice to the server and log out
    imap_session.logout()?;
    println!("ALl good!");
    // Ok(Some(body))
    Ok(())
}