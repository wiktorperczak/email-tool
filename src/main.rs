extern crate base64;
extern crate imap;
extern crate native_tls;

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

fn main() {
    let gmail_auth = GmailOAuth2 {
        user: String::from("rust1.project2@gmail.com"),
        access_token: String::from("ya29.a0AXooCgvlrTINP-lgUIMZS0ZYXEDhWaCF3uoqdOQ-BODNfG2TNmKDSWsTjFyfH7dA6nrw1pXKLG3lO9CvNn6fEZ2LyMOudRvc4mmnymtwGv4RA_t9ycs1Y1b4n-Vjxup0KT9XrErtmdqXR6MGmaJl2f2W27gIgH6SlkNJaCgYKAX8SARESFQHGX2MicG3ite7B1xHSMFN0ThySbw0171"),
    };

    let domain = "imap.gmail.com";
    let port = 993;
    let tls = native_tls::TlsConnector::builder().build().unwrap();

    let client = imap::connect((domain, port), domain, &tls).unwrap();
    // let client = imap::ClientBuilder::new("imap.gmail.com", 993)
    //     .connect()
    //     .expect("Could not connect to imap.gmail.com");

    let mut imap_session = match client.authenticate("XOAUTH2", &gmail_auth) {
        Ok(c) => c,
        Err((e, _unauth_client)) => {
            println!("error authenticating: {}", e);
            return;
        }
    };

    match imap_session.select("INBOX") {
        Ok(mailbox) => println!("{}", mailbox),
        Err(e) => println!("Error selecting INBOX: {}", e),
    };

    // let uids = imap_session.search("FLAGGED")?;
    // if uids.is_empty() {
    //     println!("No starred messages found");
    //     return Ok(());
    // }

    match imap_session.fetch("2", "body[text]") {
        Ok(msgs) => {
            for msg in msgs.iter() {
                println!("MESSAGE 2: ");
                println!("{:?}", msg);
                
                // TODO: konwersja wiadomości na czytelną postać
            }
        }
        Err(e) => println!("Error Fetching email 2: {}", e),
    };

    match imap_session.fetch("2", "RFC822") {
        Ok(msgs) => {
            for msg in msgs.iter() {
                println!("CONVERTED:");
                let body = msg.body().expect("message did not have a body!");
                let body1 = std::str::from_utf8(body)
                                        .expect("message was not valid utf-8")
                                        .to_string();
                println!("TEXT: {}", body1);
            }
        }
        Err(e) => println!("Error Fetching email 2: {}", e),
    };

    imap_session.logout().unwrap();
}