use regex::Regex;


#[derive(Debug, Clone, Eq, PartialEq, Hash)]
pub struct StarredMessage {
	pub starrer: Option<String>,
	pub stars  : u64,
	pub sender : String,
	pub message: String,
}

impl StarredMessage {
	pub fn from_message_content(message: &str, starrer: Option<String>) -> Option<StarredMessage> {
		Self::regex().captures(message).map(move |captures| StarredMessage{
			starrer: starrer,
			stars  : 1u64,
			sender : captures[1].to_string(),
			message: captures[2].to_string(),
		})
	}


	fn regex() -> Regex {
		Regex::new(r#"[[:blank:]]*<([[:alnum:]\[\]`_^{|}-]{0,16})>[[:blank:]]*(.+)[[:blank:]]*"#).unwrap()
	}
}


mod tests {
	use starred_message::StarredMessage;


	#[test]
	fn message_extracts_correctly() {
		let msg = StarredMessage::from_message_content("<nabijaczleweli> I only clean 'round these parts", None).unwrap();
		assert_eq!(msg.sender, "nabijaczleweli");
		assert_eq!(msg.message, "I only clean 'round these parts");
	}

	#[test]
	fn message_fails_properly() {
		assert_eq!(StarredMessage::from_message_content("<nabijaczleweclean 'round these parts", None), None);
		assert_eq!(StarredMessage::from_message_content("<nabijaczleweli>", None), None);
	}

	#[test]
	fn message_propagates_starrer() {
		assert_eq!(StarredMessage::from_message_content("<nabijaczleweli> I only clean 'round these parts", None).unwrap().starrer, None);
		assert_eq!(StarredMessage::from_message_content("<nabijaczleweli> I only clean 'round these parts",
		                                                Some("thecoshman".to_string())).unwrap().starrer, Some("thecoshman".to_string()));
	}

	#[test]
	fn message_defaults_to_1_star() {
		assert_eq!(StarredMessage::from_message_content("<nabijaczleweli> I only clean 'round these parts", None).unwrap().stars, 1);
	}
}
