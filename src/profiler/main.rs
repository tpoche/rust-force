extern mod xml;

use std::io::File;
use std::io::Reader;
use std::path::Path;
use profile::{FieldPermission, ObjectPermission, RecordTypeVisibility, 
			  Profile, get_element_value};

pub mod profile;

fn main() {
	let args = std::os::args();
	if args.len() != 2 {
		// file must be specified in args
		// ie ~/Workspaces/Ruby/ProfileParser/profiles/Accounting.profile
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
				Ok(Some(e)) => {
					if e.name == ~"Profile" {
						handle_root(&e)
					}
					else {
						handle_element(&e)
					}
				},
				Ok(None) => (),
				Err(e) => println!("{}", e),
			},
			Err(e) => println!("Line: {} Column: {} Msg: {}", e.line, e.col, e.msg),
		}
	});
}

fn handle_root(e: &xml::Element) {
	println!("Root found: {}", e.name);
	let mut p = Profile::new();
	let mut fperms = ~[];
	let mut operms = ~[];
	let mut rtvis  = ~[];
	for x in e.children.iter() {
		match *x {
			xml::Element(ref e) => {
				match e.name {
					~"fieldPermissions" => fperms.push(FieldPermission::from_xml(e)),
					~"objectPermissions" => operms.push(ObjectPermission::from_xml(e)),
					~"recordTypeVisibilities" => rtvis.push(RecordTypeVisibility::from_xml(e)),
					~"userLicense" => p.set_user_license(get_element_value(e)),
					_ => handle_element(e),
				}
			},
			_ => println!("handle_root received other type!")
		}
	}

	if !fperms.is_empty() {
		println!("Field Perm Total: {}", fperms.len());
		p.push_field_perms(fperms);
	}

	if !operms.is_empty() {
		println!("Object Perm Total: {}", operms.len());
		p.push_object_perms(operms);
	}

	if !rtvis.is_empty() {
		println!("Record Type Visibility Total: {}", rtvis.len());
		p.push_record_types(rtvis);
	}

	println!("{}", p.to_str());
}

fn handle_element(e: &xml::Element) {
	println!("Element Name: {}", e.name);
	if e.children.len() > 0 {
		for x in e.children.iter() {
			match *x {
				xml::Element(ref e) if e.name == ~"fieldPermissions" =>
					println!("{}", get_element_value(e)),
				xml::Element(ref e) => handle_element(e),
				xml::CharacterNode(ref cn) => println!("Charnode: {}", *cn),
				xml::CDATANode(ref cd) => println!("CDATAnode: {}", *cd),
				xml::CommentNode(ref co) => println!("Comment: {}", *co),
				//println!("Child Element: {}", handle_element(e)),
				_ => println!("No more elements found")
			}
		}
	}
}



fn read_file(filepath: &str) -> Option<~str> {
	let f = &Path::new(filepath);
	if !f.exists() {
		println!("File '{}' does not exist", filepath);
		return None;
	}

	Some(File::open(f).read_to_str())
}