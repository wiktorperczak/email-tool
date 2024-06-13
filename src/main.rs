extern crate base64;
extern crate imap;

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
        user: String::from("<e-mail>"),
        access_token: String::from("<access token>"),
    };

    let client = imap::ClientBuilder::new("imap.gmail.com", 993)
        .connect()
        .expect("Could not connect to imap.gmail.com");

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