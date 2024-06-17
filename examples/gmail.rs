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

fn main() -> imap::error::Result<()> {
    let gmail_auth = GmailOAuth2 {
        user: String::from("rust1.project2@gmail.com"),
        access_token: String::from("ya29.a0AXooCgvNwe-xI6zkebpdQ1nwoIEVNNDwB-Igb7xcES8qmImL4sk_wq8vhXC4zB4Oxz3cg1RgYo67ah79yMH5BULMc4cUMc00Lh5JKylAFc_3sOnTEWCAU_Q-tLtKluaS3BJhAHJvoS2SkhjEMBQdkzte37NYmXLY45NFaCgYKAWQSARESFQHGX2MiWzVC75_9Yyya2rANk4nyRw0171"),
    };

    let domain = "imap.gmail.com";
    let port = 993;
    let tls = native_tls::TlsConnector::builder().build().unwrap();

    let client = imap::connect((domain, port), domain, &tls).unwrap();
    // let client = imap::ClientBuilder::new("imap.gmail.com", 993)
    //     .connect()
    //     .expect("Could not connect to imap.gmail.com");

    let mut imap_session = client.authenticate("XOAUTH2", &gmail_auth).map_err(|e| {
        eprintln!("Login error!");
        e.0
    })?;


    let folders = imap_session.list(None, Some("*"))?;
    for folder in folders.iter() {
        println!("{}", folder.name());
    }


    imap_session.select("[Gmail]/Starred").map_err(|e| {
        println!("Error selecting INBOX\n: {}", e);
        e
    })?;

    let uids = imap_session.search("ALL")?;

    // Pobieranie i wyświetlanie każdej wiadomości
    for uid in uids.iter() {
        let messages = imap_session.fetch(uid.to_string(), "RFC822")?;
        for message in messages.iter() {
            if let Some(body) = message.body() {
                let body_str = std::str::from_utf8(body).expect("Wiadomość nie jest poprawnym UTF-8");
                println!("{}", body_str);
            }
        }
    }

    imap_session.logout().unwrap();
    Ok(())
}