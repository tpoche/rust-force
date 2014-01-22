extern mod xml;
use std::io::File;
use std::io::Reader;
use std::path::Path;

fn main() {
	let filename = ~"/Users/tpoche/Workspaces/Ruby/ProfileParser/profiles/Accounting.profile";
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