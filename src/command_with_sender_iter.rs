use irc::client::prelude::*;
use std::boxed::Box;
use std::io;


pub struct CommandWithSenderIter<'a> {
	iter: Box<Iterator<Item=io::Result<Message>> + 'a>,
}

pub trait CommandWithSenderIterable<'a>: Server {
	fn iter_cmd_sender(&'a self) -> CommandWithSenderIter<'a> {
		CommandWithSenderIter{
			iter: self.iter(),
		}
	}
}


impl<'a> Iterator for CommandWithSenderIter<'a> {
	type Item = io::Result<(Command, Option<String>)>;

	fn next(&mut self) -> Option<Self::Item> {
		self.iter.next().map(|msg: io::Result<Message>| {
			msg.map(|msg: Message| (
				msg.source_nickname().map(String::from),
				Ok(msg),
			)).and_then(|(nick, msg): (Option<String>, io::Result<Message>)|
				msg.map(|m: Message| (
					m.command,
					nick,
				))
			)
		})
	}
}

impl<'a, Concrete: Server> CommandWithSenderIterable<'a> for Concrete {}
