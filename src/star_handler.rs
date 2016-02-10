use starred_message::StarredMessage;
use rand::{Rng, thread_rng};
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
		static EMPTY_STARBOARD_RESPONSES: &'static [&'static str] = &[
			"There's nothing here. Star something!",
			"Michael, there's no stars here, mate. It's an empty board!",
			"ALL YOUR STARS ARE BELONG TO U... You don't have any stars? Oh, very well.",
			">tfw no stars",
		];

		if self.starred.is_empty() {
			self.server.send_notice(to, thread_rng().choose(EMPTY_STARBOARD_RESPONSES).unwrap()).unwrap();
		} else {
			let mut sorted_stars = self.starred.clone();
			sorted_stars.sort_by(|lhs, rhs| lhs.stars().cmp(&rhs.stars()));

			for message in sorted_stars.iter().take(10) {
				self.server.send_notice(to, &*&format!("{}", message)).unwrap();
			}
		}
	}

	pub fn add_star(&mut self, starrer: &str, message: &str) {
		if let Some(star_message) = StarredMessage::from_message_content(message, starrer) {
			if let Some(star_message) = self.maybe_increase_starcount(star_message) {
				self.starred.push(star_message);
			}
		}
	}

	pub fn remove_star(&mut self, starrer: &str, message: &str) {
		if let Some(star_message) = StarredMessage::from_message_content(message, starrer) {
			if let Some(index) = self.maybe_decrease_starcount(star_message) {
				self.starred.swap_remove(index);
			}
		}
	}


	/// Incease the star count on an existing equivalent message or return the message for it to be added.
	fn maybe_increase_starcount(&mut self, to_star: StarredMessage) -> Option<StarredMessage> {
		match self.starred.iter_mut().find(|fmsg| (&fmsg.sender, &fmsg.message) == (&to_star.sender, &to_star.message)) {
			Some(ref mut existing_message) => {
				existing_message.starrers.extend(to_star.starrers);
				None
			},
			None => Some(to_star),
		}
	}

	/// Decrease the star count and remove the starrer from an existing equivalent message and return the message's index if it has 0 stars after decreasing.
	fn maybe_decrease_starcount(&mut self, to_unstar: StarredMessage) -> Option<usize> {
		self.starred.iter_mut().enumerate().find(
			|fmsg| (&fmsg.1.sender, &fmsg.1.message) == (&to_unstar.sender, &to_unstar.message)
		).and_then(|(idx, ref mut existing_message)|
			if existing_message.starrers.remove(to_unstar.starrers.iter().next().unwrap()) && existing_message.stars() == 0 {
				Some(idx)
			} else {
				None
			}
		)
	}
}
