use starred_message::StarredMessage;
use irc::client::prelude::*;
use irc::client::server::NetIrcServer;
use std::sync::Arc;


pub struct StarHandler {
	server : Arc<NetIrcServer>,
	starred: Vec<StarredMessage>,
}

impl StarHandler {
	pub fn new(server: &Arc<NetIrcServer>) -> StarHandler {
		StarHandler{
			server : server.clone(),
			starred: Vec::new(),
		}
	}


	pub fn dump(&self, onto: &str) {
		self.server.send_privmsg(onto, &*&format!("{:?}", self.starred)).unwrap()
	}

	pub fn show_board(&self, to: &str) {
		let mut sorted_stars = self.starred.clone();
		sorted_stars.sort_by(|lhs, rhs| lhs.stars.cmp(&rhs.stars));

		for message in sorted_stars.iter().take(10) {
			self.server.send_notice(to, &*&format!("{}", message)).unwrap();
		}
	}

	pub fn add_star(&mut self, starrer: &str, message: &str) {
		if let Some(star_message) = StarredMessage::from_message_content(message, starrer) {
			if match self.starred.iter_mut().find(|fmsg| (&fmsg.sender, &fmsg.message) == (&star_message.sender, &star_message.message)) {
				Some(ref mut existing_message) => {
					existing_message.stars += 1;
					existing_message.starrers.extend(star_message.starrers.clone());
					false
				},
				None => true,
			} {
				self.starred.push(star_message);  // Can't do it in match arm because it'd borrow starred as &mut twice
			}
		}
	}

	pub fn remove_star(&mut self, starrer: &str, message: &str) {
		if let Some(star_message) = StarredMessage::from_message_content(message, starrer) {
			if let Some(index) = match self.starred.iter_mut().enumerate().find(
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
				self.starred.swap_remove(index);  // Can't do it in match arm because it'd borrow starred as &mut twice
			}
		}
	}
}
