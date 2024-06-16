mod client;
mod list_messages;
mod email_parser;
mod save_to_txt;
mod statistics;

use clearscreen::clear;

fn main() {
    clear().expect("Failed to clear screen");
    let _ = client::run();
}