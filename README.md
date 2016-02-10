# IRC-starbot [![Build Status](https://travis-ci.org/nabijaczleweli/IRC-starbot.svg?branch=master)](https://travis-ci.org/nabijaczleweli/IRC-starbot) [![Licence](https://img.shields.io/badge/license-MIT-blue.svg?style=flat)](LICENSE)
An IRC bot for handling message stars, [Snackchat](http://chat.stackoverflow.com)-style

## Howto

By default the bot connects to `#loungecpp@chat.freenode.net`, edit `src/main.rs` to change that.

After tailoring the bot for your specific needs, running the binary (`cargo run`) will start and connect the bot to the IRC server and channel specified.

## Bot commands

  * `/msg StarBot help` — display level 0 help
  * `/msg StarBot board` — pretty-print the top 10 starboard entries
  * `/msg StarBot add MESSAGE` — add your star to `MESSAGE`
  * `/msg StarBot remove MESSAGE` — remove your star from `MESSAGE`
  * `/msg StarBot _help` — display level 1 help
  * `/msg StarBot _dump` — dump the raw starboard contents, useless for the normal end-user
  * `/msg StarBot __help` — display level 2 help
  * `/msg StarBot Navaer` — murder StarBot violently
  * `Navaer, StarBot` — murder StarBot violently (*nota bene*: doesn't need to be PRIVMSGd directly to StarBot)

### Format

  * `MESSAGE` — `<SENDER> CONTENT`
  * `SENDER` — a valid username, as per [RFC 2812](https://tools.ietf.org/html/rfc2812#section-2.3.1)
  * `CONTENT` — message content

For example:

  * `MESSAGE` — `<nabijaczleweli> I only clean 'round these parts`
  * `SENDER` — `nabijaczleweli`
  * `CONTENT` — `I only clean 'round these parts`
