use command_with_sender_iter::CommandWithSenderIterable;
use starred_message::StarredMessage;
use irc::client::prelude::*;
use irc::client::server::NetIrcServer;


pub struct IrcResponder {
	server: NetIrcServer,
}

impl IrcResponder {
	pub fn from_config(config: Config) -> IrcResponder {
		let server = IrcServer::from_config(config).unwrap();
		server.identify().unwrap();

		IrcResponder{
			server: server,
		}
	}

	pub fn handle(&mut self) {
		let mut starred: Vec<StarredMessage> = Vec::new();

		for message in self.server.iter_cmd_sender() {
			match message {
				Ok((Command::JOIN(_, _, _), Some(sender))) =>
					if sender == "StarBot" {
						println!("I'm in")
					},
				Ok((Command::PRIVMSG(target, msg), sender)) => {
					match (&target[..], &msg[..], sender.as_ref().map(|s| &s[..])) {
						(_,         "Navaer, StarBot",  _) => self.quit(),
						("StarBot", "Navaer",           _) => self.quit(),
						("StarBot", "board", Some(sender)) => {
							let mut sorted_stars = starred.clone();
							sorted_stars.sort_by(|lhs, rhs| lhs.stars.cmp(&rhs.stars));

							for message in sorted_stars.iter().take(10) {
								self.server.send_notice(&*&sender, &*&format!("{}", message)).unwrap();
							}
						},
						("StarBot", "_dump", Some(sender)) => self.server.send_privmsg(&*&sender, &*&format!("{:?}", starred)).unwrap(),
						("StarBot", "help",  Some(sender)) => {
							self.server.send_notice(&*&sender, r#"Cummands, level 0:"#).unwrap();
							self.server.send_notice(&*&sender, r#"  "add <" username ">" message content -- add your star to a message"#).unwrap();
							self.server.send_notice(&*&sender, r#"  "remove <" username ">" message content -- remove your star from a message"#).unwrap();
							self.server.send_notice(&*&sender, r#"  "help" -- send this help notice to sender"#).unwrap();
							self.server.send_notice(&*&sender, r#"  "board" -- pretty-print the starboard, snackchat-style"#).unwrap();
							self.server.send_notice(&*&sender, r#"Execute via sending a PRIVMSG (as by "/msg") to StarBot"#).unwrap();
						},
						("StarBot", "_help", Some(sender)) => {
							self.server.send_notice(&*&sender, r#"Cummands, level 1:"#).unwrap();
							self.server.send_notice(&*&sender, r#"  "Navaer" -- murder StarBot"#).unwrap();
							self.server.send_notice(&*&sender, r#"  "_dump" -- dump raw star data to sender"#).unwrap();
							self.server.send_notice(&*&sender, r#"Execute via sending a PRIVMSG (as by "/msg") to StarBot"#).unwrap();
						},
						("StarBot", "__help", Some(sender)) => {
							self.server.send_notice(&*&sender, r#"Cummands, level 2:"#).unwrap();
							self.server.send_notice(&*&sender, r#"  "Navaer, StarBot" -- murder StarBot"#).unwrap();
							self.server.send_notice(&*&sender, r#"Execute via sending a PRIVMSG any channel StarBot is listening to"#).unwrap();
						},
						("StarBot", msg, Some(sender)) => {
							if msg.starts_with("add ") {
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
							} else if msg.starts_with("remove ") {
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
							}
						},
						_ => (),
					}
				},
				_ => (),
			}
		}
	}

	fn quit(&self) {
		self.server.send_quit("Mára mesta").unwrap();
	}
}
