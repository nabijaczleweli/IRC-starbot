extern crate regex;
extern crate irc;

mod starred_message;

use irc::client::prelude::*;
use std::default::Default;
use starred_message::StarredMessage;


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
			let sender = message.get_source_nickname().map(String::from);

			match Command::from_message_io(Ok(message)) {
				Ok(Command::JOIN(_, _, _))        => println!("I'm in"),
				Ok(Command::PRIVMSG(target, msg)) => {
					let to_self = target == "NabBot";

					if (to_self && msg == "Navaer") || msg == "Navaer, NabBot" {
						server.send_quit("Mára mesta").unwrap();
					} else if to_self && msg.starts_with("add ")  {
						println!("{:?}", StarredMessage::from_message_content(&msg[4..], sender));
					}
				}
				_ => (),
			}
		}
	}
}
