mod client;
mod list_messages;
mod email_parser;
mod save_to_txt;

use clearscreen::clear;
use std::process::Command;

fn main() {
    clear().expect("failed to clear screen");
    let _ = client::run();
}