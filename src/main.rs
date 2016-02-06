extern crate regex;
extern crate irc;

mod starred_message;

use irc::client::prelude::*;
use std::default::Default;
use starred_message::StarredMessage;


fn main() {
	let server = IrcServer::from_config(Config{
		owners   : Some(vec!["nabijaczleweli".to_string()]),
		nickname : Some("StarBot".to_string()),
		username : Some("StarBot".to_string()),
		realname : Some("Bot for handling stars, Snackchat-style".to_string()),
		server   : Some("chat.freenode.net".to_string()),
		use_ssl  : Some(true),
		channels : Some(vec!["#loungecpp".to_string()]),
		user_info: Some("Bot for handling stars, Snackchat-style. /msg it with \"help\" for guidance".to_string()),
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
					let to_self = target == "StarBot";

					if (to_self && msg == "Navaer") || msg == "Navaer, StarBot" {
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
					} else if to_self && msg.starts_with("remove ") {
						if let Some(star_message) = StarredMessage::from_message_content(&msg[7..], sender) {
							if let Some(index) = match starred.iter_mut().enumerate().find(
								|fmsg| (&fmsg.1.sender, &fmsg.1.message) == (&star_message.sender, &star_message.message)
							) {
								Some((idx, ref mut existing_message)) =>
									if let Some(starrer_pos) = existing_message.starrers.iter().position(|starrer| starrer == &existing_message.starrers[0]) {
										existing_message.stars -= 1;
										existing_message.starrers.swap_remove(starrer_pos);
										Some(idx)
									} else {
										None
									},
								None => None,
							} {
								starred.swap_remove(index);  // Can't do it in match arm because it'd borrow starred as &mut twice
							}
						}
					} else if to_self && msg.ends_with("help") && sender.is_some() {
						let sender = &*&sender.unwrap();
						let msg = msg.trim();

						let level = msg.find("help").unwrap();
						if msg[..level].chars().all(|c| c == '_') {
							server.send_notice(sender, r#"Cummands, level 0:"#).unwrap();
							server.send_notice(sender, r#"  "add <" username ">" message content -- add your star to a message"#).unwrap();
							server.send_notice(sender, r#"  "help" -- send this help notice to sender"#).unwrap();
							server.send_notice(sender, r#"  "board" -- pretty-print the starboard, snackchat-style"#).unwrap();
							server.send_notice(sender, r#"Execute via sending a PRIVMSG (as by "/msg") to StarBot"#).unwrap();
							if level >= 1 {
								server.send_notice(sender, r#"Cummands, level 1:"#).unwrap();
								server.send_notice(sender, r#"  "Navaer" -- murder StarBot"#).unwrap();
								server.send_notice(sender, r#"  "_dump" -- dump raw star data to sender"#).unwrap();
								server.send_notice(sender, r#"Execute via sending a PRIVMSG (as by "/msg") to StarBot"#).unwrap();
							}
							if level >= 2 {
								server.send_notice(sender, r#"Cummands, level 2:"#).unwrap();
								server.send_notice(sender, r#"  "Navaer, StarBot" -- murder StarBot"#).unwrap();
								server.send_notice(sender, r#"Execute via sending a PRIVMSG any channel StarBot is listening to"#).unwrap();
							}
						}
					} else if to_self && msg == "board" && sender.is_some() {
						let sender = &*&sender.unwrap();
						let mut sorted_stars = starred.clone();
						sorted_stars.sort_by(|lhs, rhs| lhs.stars.cmp(&rhs.stars));

						for message in sorted_stars.iter().take(10) {
							server.send_notice(sender, &*&format!("{}", message)).unwrap();
						}
					} else if to_self && msg == "_dump" && sender.is_some() {
						server.send_privmsg(&*&sender.unwrap(), &*&format!("{:?}", starred)).unwrap();
					}
				}
				_ => (),
			}
		}
	}
}
