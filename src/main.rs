extern crate irc;

use irc::client::prelude::*;
use std::default::Default;


fn main() {
	let server = IrcServer::from_config(Config{
		owners   : Some(vec!["nabijaczleweli".to_string()]),
		nickname : Some("NabBot".to_string()),
		username : Some("NabBot".to_string()),
		realname : Some("наб's IRC bot".to_string()),
		server   : Some("chat.freenode.net".to_string()),
		use_ssl  : Some(true),
		channels : Some(vec!["#loungecpp".to_string()]),
		user_info: Some("наб's IRC bot".to_string()),
		..Default::default()
	}).unwrap();
	server.identify().unwrap();

	for message in server.iter() {
		if let Ok(message) = message {
			let source_nickname = message.get_source_nickname().map(String::from);

			if let Ok(Command::PRIVMSG(target, msg)) = Command::from_message_io(Ok(message)) {
				if (target == "NabBot" && msg == "Navaer") || msg == "Navaer, NabBot" {
					server.send_privmsg(&*&source_nickname.unwrap_or(target), "Mára mesta").unwrap();
					server.send_quit("Mára mesta").unwrap();
				} else if msg.contains("isn't") || msg.contains("is not") {
					println!("{}: {:?} -> Not your face", source_nickname.as_ref().unwrap_or(&"???".to_string()), msg);
					server.send_privmsg(&target, "Quite unlike your face").unwrap();
				} else if msg.contains("sucks") || msg.contains("is") {
					println!("{}: {:?} -> Your face", source_nickname.as_ref().unwrap_or(&"???".to_string()), msg);
					server.send_privmsg(&target, "Much like your face").unwrap();
				}
			}
		}
	}
}
