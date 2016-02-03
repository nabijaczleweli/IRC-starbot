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


	let mut starred: Vec<StarredMessage> = Vec::new();

	for message in server.iter() {
		if let Ok(message) = message {
			let sender = message.get_source_nickname().map(String::from);

			match Command::from_message_io(Ok(message)) {
				Ok(Command::JOIN(_, _, _))        => println!("I'm in"),
				Ok(Command::PRIVMSG(target, msg)) => {
					let to_self = target == "NabBot";

					if (to_self && msg == "Navaer") || msg == "Navaer, NabBot" {
						server.send_quit("Mára mesta").unwrap();
					} else if to_self && msg.starts_with("add ") {
						if let Some(star_message) = StarredMessage::from_message_content(&msg[4..], sender) {
							if match starred.iter_mut().find(|fmsg| (&fmsg.sender, &fmsg.message) == (&star_message.sender, &star_message.message)) {
								Some(ref mut existing_message) => {
									existing_message.stars += 1;
									existing_message.starrers.extend(star_message.starrers.clone());
									false
								},
								None => true,
							} {
								starred.push(star_message);  // Can't do it in match arm because it'd borrow starred as &mut twice
							}
						}
					} else if to_self && msg == "help" && sender.is_some() {
						let sender = &*&sender.unwrap();

						server.send_notice(sender, r#"Cummands:"#).unwrap();
						server.send_notice(sender, r#"  "add <" username ">" message content -- add your star to a message"#).unwrap();
						server.send_notice(sender, r#"  "help" -- send this help notice to sender"#).unwrap();
						server.send_notice(sender, r#"  "dump" -- dump all star data to sender"#).unwrap();
						server.send_notice(sender, r#"Execute via sending a PRIVMSG (as by "/msg") to NabBot"#).unwrap();
					} else if to_self && msg == "dump" && sender.is_some() {
						server.send_privmsg(&*&sender.unwrap(), &*&format!("{:?}", starred)).unwrap();
					}
				}
				_ => (),
			}
		}
	}
}
