# Email searching and statistics tool

Program uses IMAP protocol to connect with email, and provides many functions. User can download messages from given folder, from given user or all unread messages. Messages are saved to .txt file. Additionally program generates statistics of a given account, such as: total number of messages, number of messages in given folder, and most active message senders.

Note: Currently, only mails from gmails are allowed.

```
[dependencies]
imap = "2.4.1"
native-tls = "0.2"
base64 = "0.21"
mailparse = "0.13.1"
chrono = "0.4"
rpassword = "0.0.4"
clearscreen = "3.0.0"
```
