use regex::Regex;
use std::collections::btree_set::BTreeSet;
use std::fmt;


#[derive(Debug, Clone, Eq, PartialEq, Hash)]
pub struct StarredMessage {
	pub starrers: BTreeSet<String>,
	pub sender  : String,
	pub message : String,
}

impl StarredMessage {
	pub fn from_message_content(message: &str, starrer: &str) -> Option<StarredMessage> {
		let mut starrers = BTreeSet::new();
		starrers.insert(starrer.to_string());

		Self::regex().captures(message).map(move |captures| StarredMessage{
			starrers: starrers,
			sender  : captures[1].to_string(),
			message : captures[2].to_string(),
		})
	}

	pub fn stars(&self) -> u64 {
		self.starrers.len() as u64
	}


	fn regex() -> Regex {
		Regex::new(r#"[[:blank:]]*<([[:alnum:]\[\]`_^{|}-]{0,16})>[[:blank:]]*(.+)[[:blank:]]*"#).unwrap()
	}
}

impl fmt::Display for StarredMessage {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		write!(f, "{}★ <{}> {}", self.stars(), self.sender, self.message)
	}
}


#[cfg(test)]
mod tests {
	use starred_message::StarredMessage;
	use std::collections::btree_set::BTreeSet;


	#[test]
	fn message_extracts_correctly() {
		let msg = StarredMessage::from_message_content("<nabijaczleweli> I only clean 'round these parts", "thecoshman").unwrap();
		assert_eq!(msg.sender, "nabijaczleweli");
		assert_eq!(msg.message, "I only clean 'round these parts");
	}

	#[test]
	fn message_fails_properly() {
		assert_eq!(StarredMessage::from_message_content("<nabijaczleweclean 'round these parts", "thecoshman"), None);
		assert_eq!(StarredMessage::from_message_content("<nabijaczleweli>", "thecoshman"), None);
	}

	#[test]
	fn message_propagates_starrer() {
		let mut starrers = BTreeSet::new();
		starrers.insert("thecoshman".to_string());

		assert_eq!(StarredMessage::from_message_content("<nabijaczleweli> I only clean 'round these parts", "thecoshman").unwrap().starrers, starrers);
	}

	#[test]
	fn stars_grow_with_starrers() {
		let mut message = StarredMessage::from_message_content("<nabijaczleweli> I only clean 'round these parts", "thecoshman").unwrap();
		assert_eq!(message.stars(), 1);

		for (i, n) in ["CatPlusPlus", "pirate", "ely-se"].iter().enumerate() {
			message.starrers.insert(n.to_string());
			assert_eq!(message.stars(), (i + 2) as u64);
		}
	}

	#[test]
	fn format() {
		assert_eq!(format!("{}", StarredMessage::from_message_content("<nabijaczleweli> I only clean 'round these parts", "thecoshman").unwrap()),
		           "1★ <nabijaczleweli> I only clean 'round these parts".to_string());
	}
}
