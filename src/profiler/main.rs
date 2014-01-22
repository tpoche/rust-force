extern mod xml;
use std::io::File;
use std::io::Reader;
use std::path::Path;

fn main() {
	let args = std::os::args();
	if args.len() != 2 {
		/// file must be specified in args
		/// ie ~/Workspaces/Ruby/ProfileParser/profiles/Accounting.profile
		println!("Usage: {} <file>", args[0]);
		return;
	}
	
	let filename: ~str = args[1].clone();	
	let contents: ~str = match read_file(filename) {
		Some(c) => c,
		None => ~"Error reading file contents"
	};
	
	let mut p = xml::Parser::new();
	let mut e = xml::ElementBuilder::new();

	p.parse_str(contents, |event| {
		match event {
			Ok(event) => match e.push_event(event) {
				Ok(Some(e)) => println!("{}", e),
				Ok(None) => (),
				Err(e) => println!("{}", e),
			},
			Err(e) => println!("Line: {} Column: {} Msg: {}", e.line, e.col, e.msg),
		}
	});
}

fn read_file(filepath: &str) -> Option<~str> {
	let f = &Path::new(filepath);
	if !f.exists() {
		println!("File '{}' does not exist", filepath);
		return None;
	}

	Some(File::open(f).read_to_str())
}