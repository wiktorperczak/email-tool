mod client;
mod list_messages;
mod email_parser;
mod save_to_txt;

fn main() {
    let _ = client::run();
}