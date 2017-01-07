#[macro_use]
extern crate clap;
use clap::{Arg, App};

fn main() {
	println!("Hello, world!");

	let matches = App::new("systemd-timeline")
		.version("0.1.0")
		.author("TesX <tesfabpel@gmail.com>")
		.about("A little utility to display a timeline of systemd units of a specific boot")
		.arg(Arg::with_name("boot")
			.short("b")
			.long("boot")
			.takes_value(true)
			.help("Shows the timeline for a specific boot instead of the current one"))
		.get_matches_safe()
		.unwrap_or_else(|e| e.exit());

	let boot = value_t!(matches.value_of("boot"), i32).unwrap_or(0);

	println!("boot: {:?}", boot);
}
