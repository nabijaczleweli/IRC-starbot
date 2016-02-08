use irc::client::prelude::*;
use irc::client::server::ServerIterator;
use std::io;


pub struct CommandWithSenderIter<'a, T: IrcRead + 'a, U: IrcWrite + 'a> {
	iter: ServerIterator<'a, T, U>,
}

pub trait CommandWithSenderIterable<'a, T: IrcRead + 'a, U: IrcWrite + 'a>: Server<'a, T, U> {
	fn iter_cmd_sender(&'a self) -> CommandWithSenderIter<'a, T, U> {
		CommandWithSenderIter{
			iter: self.iter(),
		}
	}
}


impl<'a, T: IrcRead + 'a, U: IrcWrite + 'a> Iterator for CommandWithSenderIter<'a, T, U> {
	type Item = io::Result<(Command, Option<String>)>;

	fn next(&mut self) -> Option<Self::Item> {
		self.iter.next().map(|msg: io::Result<Message>| {
			msg.map(|msg: Message| (
				msg.get_source_nickname().map(String::from),
				Ok(msg),
			)).and_then(|(nick, msg): (Option<String>, io::Result<Message>)|
				Command::from_message_io(msg).map(|cmd: Command| (
					cmd,
					nick,
				))
			)
		})
	}
}

impl<'a, T: IrcRead + 'a, U: IrcWrite + 'a, Concrete: Server<'a, T, U>> CommandWithSenderIterable<'a, T, U> for Concrete {}
