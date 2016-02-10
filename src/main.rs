extern crate regex;
extern crate rand;
extern crate irc;

mod star_handler;
mod irc_responder;
mod starred_message;
mod command_with_sender_iter;

use irc::client::prelude::*;
use std::default::Default;


fn main() {
	irc_responder::IrcResponder::from_config(Config{
		owners   : Some(vec!["nabijaczleweli".to_string()]),
		nickname : Some("StarBot".to_string()),
		username : Some("StarBot".to_string()),
		realname : Some("Bot for handling stars, Snackchat-style".to_string()),
		server   : Some("chat.freenode.net".to_string()),
		use_ssl  : Some(true),
		channels : Some(vec!["#loungecpp".to_string()]),
		user_info: Some("Bot for handling stars, Snackchat-style. /msg it with \"help\" for guidance".to_string()),
		..Default::default()
	}).handle();
}
