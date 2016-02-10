use command_with_sender_iter::CommandWithSenderIterable;
use star_handler::StarHandler;
use irc::client::prelude::*;
use irc::client::server::NetIrcServer;
use std::sync::Arc;


pub struct IrcResponder {
	starboard: StarHandler,
	server   : Arc<NetIrcServer>,
}

impl IrcResponder {
	pub fn from_config(config: Config) -> IrcResponder {
		let server = Arc::new(IrcServer::from_config(config).unwrap());
		server.identify().unwrap();

		IrcResponder{
			starboard: StarHandler::new(&server),
			server   : server,
		}
	}

	pub fn handle(&mut self) {
		for message in self.server.iter_cmd_sender() {
			match message {
				Ok((Command::JOIN(_, _, _), Some(sender))) =>
					if sender == "StarBot" {
						println!("I'm in")
					},
				Ok((Command::PRIVMSG(target, msg), sender)) => {
					match (&target[..], &msg[..], sender.as_ref().map(|s| &s[..])) {
						(_,         "Navaer, StarBot",   _) => self.quit(),
						("StarBot", "Navaer",            _) => self.quit(),
						("StarBot", "help",   Some(sender)) => self.help(&*&sender, 0),
						("StarBot", "_help",  Some(sender)) => self.help(&*&sender, 1),
						("StarBot", "__help", Some(sender)) => self.help(&*&sender, 2),
						("StarBot", "_dump",  Some(sender)) => self.starboard.dump(sender),
						("StarBot", "board",  Some(sender)) => self.starboard.show_board(sender),
						("StarBot", msg,      Some(sender)) =>
							if msg.starts_with("add ") {
								self.starboard.add_star(sender, &msg[4..])
							} else if msg.starts_with("remove ") {
								self.starboard.remove_star(sender, &msg[7..])
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

	fn help(&self, whom: &str, level: i32) {
		self.server.send_notice(whom, r#"Cummands, level 0:"#).unwrap();
		self.server.send_notice(whom, r#"  "add <" username ">" message content -- add your star to a message"#).unwrap();
		self.server.send_notice(whom, r#"  "remove <" username ">" message content -- remove your star from a message"#).unwrap();
		self.server.send_notice(whom, r#"  "help" -- send this help notice to sender"#).unwrap();
		self.server.send_notice(whom, r#"  "board" -- pretty-print the starboard, snackchat-style"#).unwrap();
		self.server.send_notice(whom, r#"Execute via sending a PRIVMSG (as by "/msg") to StarBot"#).unwrap();

		if level >= 1 {
			self.server.send_notice(whom, r#"Cummands, level 1:"#).unwrap();
			self.server.send_notice(whom, r#"  "Navaer" -- murder StarBot"#).unwrap();
			self.server.send_notice(whom, r#"  "_dump" -- dump raw star data to sender"#).unwrap();
			self.server.send_notice(whom, r#"Execute via sending a PRIVMSG (as by "/msg") to StarBot"#).unwrap();
		}

		if level >= 2 {
			self.server.send_notice(whom, r#"Cummands, level 2:"#).unwrap();
			self.server.send_notice(whom, r#"  "Navaer, StarBot" -- murder StarBot"#).unwrap();
			self.server.send_notice(whom, r#"Execute via sending a PRIVMSG any channel StarBot is listening to"#).unwrap();
		}
	}
}
