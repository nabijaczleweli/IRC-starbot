use regex::Regex;
use std::fmt;


#[derive(Debug, Clone, Eq, PartialEq, Hash)]
pub struct StarredMessage {
	pub starrers: Vec<String>,
	pub stars   : u64,
	pub sender  : String,
	pub message : String,
}

impl StarredMessage {
	pub fn from_message_content(message: &str, starrer: &str) -> Option<StarredMessage> {
		Self::regex().captures(message).map(move |captures| StarredMessage{
			starrers: vec![starrer.to_string()],
			stars   : 1u64,
			sender  : captures[1].to_string(),
			message : captures[2].to_string(),
		})
	}


	fn regex() -> Regex {
		Regex::new(r#"[[:blank:]]*<([[:alnum:]\[\]`_^{|}-]{0,16})>[[:blank:]]*(.+)[[:blank:]]*"#).unwrap()
	}
}

impl fmt::Display for StarredMessage {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		write!(f, "{}★ <{}> {}", self.stars, self.sender, self.message)
	}
}


#[cfg(test)]
mod tests {
	use starred_message::StarredMessage;


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
		assert_eq!(StarredMessage::from_message_content("<nabijaczleweli> I only clean 'round these parts", "thecoshman").unwrap().starrers, vec!["thecoshman"]);
	}

	#[test]
	fn message_defaults_to_1_star() {
		assert_eq!(StarredMessage::from_message_content("<nabijaczleweli> I only clean 'round these parts", "thecoshman").unwrap().stars, 1);
	}

	#[test]
	fn format() {
		assert_eq!(format!("{}", StarredMessage::from_message_content("<nabijaczleweli> I only clean 'round these parts", "thecoshman").unwrap()),
		           "1★ <nabijaczleweli> I only clean 'round these parts".to_string());
	}
}
