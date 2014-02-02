/// Contains profile metadata object definitions
extern mod xml;

pub struct Profile {
	name: ~str,
	fieldPermissions: ~[FieldPermission],
	objectPermissions: ~[ObjectPermission],
	recordTypeVisibilities: ~[RecordTypeVisibility],
	userLicense: ~str,
}

pub struct FieldPermission {
	field : ~str,
	readable: bool,
	editable: bool
}

impl FieldPermission {
	pub fn new() -> FieldPermission {
		FieldPermission { field: ~"", readable: false, editable: false }
	}

	pub fn from_xml(e: &xml::Element) -> FieldPermission {
		let mut fp = FieldPermission::new();
		for x in e.children.iter() {
			match *x {
				xml::Element(ref e) => {
					match e.name {
						~"field" => { fp.set_field(get_element_value(e)); },
						~"readable" => { fp.set_readable(get_element_value(e)); },
						~"editable" => { fp.set_editable(get_element_value(e));	},
						_ => warn!("Encountered unknown element name"),
					}
				}, 
				_ => (),
			}
		}
		println!("FieldPerm parsing complete: {:?}", fp);
		fp
	}

	pub fn set_field(&mut self, f: ~str) {
		self.field = f
	}

	pub fn set_readable(&mut self, rd: ~str) -> bool {
		match from_str::<bool>(rd) {
			Some(b) => self.readable = b,
			None => self.readable = false
		} 
		self.readable
	}

	pub fn set_editable(&mut self, ed: ~str) -> bool {
		match from_str::<bool>(ed) {
			Some(b) => self.editable = b,
			None => self.editable = false
		}
		self.editable
	}
}

/// ObjectPermission definition
pub struct ObjectPermission {
	object: ~str,
	allowRead: bool,
	allowCreate: bool,
	allowEdit: bool,
	allowDelete: bool,
	viewAllRecords: bool,
	modifyAllRecords: bool
}

impl ObjectPermission {
	pub fn new() -> ObjectPermission {
		ObjectPermission {
			object: ~"",
			allowRead: false,
			allowCreate: false,
			allowEdit: false,
			allowDelete: false,
			viewAllRecords: false,
			modifyAllRecords: false
		}
	}

	pub fn from_xml(e: &xml::Element) -> ObjectPermission {
		let mut op = ObjectPermission::new();
		for x in e.children.iter() {
			match *x {
				xml::Element(ref e) => {
					match e.name {
						~"object" => { op.set_object(get_element_value(e)); },
						~"allowCreate" => { op.set_allow_create(get_element_value(e)); },
						~"allowRead" => { op.set_allow_read(get_element_value(e)); },
						~"allowEdit" => { op.set_allow_edit(get_element_value(e)); },
						~"allowDelete" => {	op.set_allow_delete(get_element_value(e)); },
						~"viewAllRecords" => { op.set_view_all(get_element_value(e)); },
						~"modifyAllRecords" => { op.set_modify_all(get_element_value(e)); },
						_ => println!("Skipped element name"),
					}
				}
				_ => (),
			}
		}
		println!("ObjectPermission parsing complete: {:?}", op);
		op
	}

	pub fn set_object(&mut self, o: ~str) {
		self.object = o;
	}

	pub fn set_allow_read(&mut self, ar: ~str) -> bool {
		match from_str::<bool>(ar) {
			Some(b) => self.allowRead = b,
			None => self.allowRead = false,
		}
		self.allowRead
	}

	pub fn set_allow_create(&mut self, ac: ~str) -> bool {
		match from_str::<bool>(ac) {
			Some(b) => self.allowCreate = b,
			None => self.allowCreate = false,
		}
		self.allowCreate
	}

	pub fn set_allow_edit(&mut self, ae: ~str) -> bool {
		match from_str::<bool>(ae) {
			Some(b) => self.allowEdit = b,
			None => self.allowEdit = false,
		}
		self.allowEdit
	}

	pub fn set_allow_delete(&mut self, ad: ~str) -> bool {
		match from_str::<bool>(ad) {
			Some(b) => self.allowDelete = b,
			None => self.allowDelete = false,
		}
		self.allowDelete
	}

	pub fn set_view_all(&mut self, va: ~str) -> bool {
		match from_str::<bool>(va) {
			Some(b) => self.viewAllRecords = b,
			None => self.viewAllRecords = false,
		}
		self.viewAllRecords
	}

	pub fn set_modify_all(&mut self, ma: ~str) -> bool {
		match from_str::<bool>(ma) {
			Some(b) => self.modifyAllRecords = b,
			None => self.modifyAllRecords = false,
		}
		self.modifyAllRecords
	}
}

/// RecordTypeVisibility definition
pub struct RecordTypeVisibility {
	recordType: ~str,
	default: bool,
	visible: bool,
}

impl RecordTypeVisibility {
	pub fn new() -> RecordTypeVisibility {
		RecordTypeVisibility {
			recordType: ~"",
			default: false,
			visible: false,
		}
	}

	pub fn from_xml(e: &xml::Element) -> RecordTypeVisibility {
		let mut rtv = RecordTypeVisibility::new();
		for x in e.children.iter() {
			match *x {
				xml::Element(ref e) => {
					match e.name {
						~"recordType" => { rtv.set_record_type(get_element_value(e)); },
						~"default" => { rtv.set_default(get_element_value(e)); },
						~"visible" => { rtv.set_visible(get_element_value(e)); },
						_ => warn!("Invalid element name"),
					}
				},
				_ => (),
			}
		}
		println!("RecordTypeVisibility parsing complete: {:?}", rtv);
		rtv
	}

	pub fn set_record_type(&mut self, rt: ~str) {
		self.recordType = rt
	}

	pub fn set_default(&mut self, def: ~str) -> bool {
		match from_str::<bool>(def) {
			Some(b) => self.default = b,
			None => self.default = false,
		}
		self.default
	}

	pub fn set_visible(&mut self, vis: ~str) -> bool {
		match from_str::<bool>(vis) {
			Some(b) => self.visible = b,
			None => self.visible = false,
		}
		self.visible
	}
}

/// helper methods
pub fn get_element_value(e: &xml::Element) -> ~str {
	let mut value = ~"";
	for x in e.children.iter() {
		match *x {
			xml::CharacterNode(ref c) => value.push_str(*c),
			_ => println!("No CharacterNode found!")
		}
	}
	value
}