use regex::Regex;


#[derive(Debug, Clone, Eq, PartialEq, Hash)]
pub struct StarredMessage {
	pub starrer: Option<String>,
	pub stars  : u64,
	pub sender : String,
	pub message: String,
}

impl StarredMessage {
	pub fn from_message_content(message: String, starrer: Option<String>) -> Option<StarredMessage> {
		Self::regex().captures(&*&message).map(move |captures| StarredMessage{
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
