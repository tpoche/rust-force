/// Contains profile metadata object definitions to support parsing
/// responses from the Salesforce Metadata API
extern mod xml;

#[deriving(Clone, Eq)]
pub struct Profile {
	name: ~str,
	fieldPermissions: ~[FieldPermission],
	objectPermissions: ~[ObjectPermission],
	recordTypeVisibilities: ~[RecordTypeVisibility],
	userLicense: ~str,
}

impl Profile {
	pub fn new() -> Profile {
		Profile {
			name: ~"",
			fieldPermissions: ~[],
			objectPermissions: ~[],
			recordTypeVisibilities: ~[],
			userLicense: ~"",
		}
	}

	pub fn set_user_license(&mut self, lic: &str) {
		if lic.len() > 0 {
			self.userLicense = lic.into_owned();
		}

	}

	pub fn push_field_perms(&mut self, fps: ~[FieldPermission]) {
		if fps.len() > 0 {
			let mut count = 0;
			for fp in fps.iter() {
				self.fieldPermissions.push(fp.clone());
				count += 1;
			}
			println!("{} field permissions added to profile", count);
		}
	}

	pub fn push_object_perms(&mut self, ops: ~[ObjectPermission]) {
		if ops.len() > 0 {
			let mut count = 0;
			for op in ops.iter() {
				self.objectPermissions.push(op.clone());
				count += 1;
			}
			println!("{} object permissions added to profile", count);
		}
	}

	pub fn push_record_types(&mut self, rtvs: ~[RecordTypeVisibility]) {
		if rtvs.len() > 0 {
			let mut count = 0;
			for rtv in rtvs.iter() {
				self.recordTypeVisibilities.push(rtv.clone());
				count += 1;
			}
			println!("{} record type visibilities added to profile", count);
		}
	}
}

impl ToStr for Profile {
	fn to_str(&self) -> ~str {
		let mut srepr = ~"Profile: \n";
		if self.name.len() > 0 {
			srepr.push_str(format!("Name: {}\n", self.name));
		}

		if self.userLicense.len() > 0 {
			srepr.push_str(format!("UserLicense: {}\n", self.userLicense));
		}

		if self.fieldPermissions.len() > 0 {
			for fp in self.fieldPermissions.iter() {
				srepr.push_str("FieldPermission: \n");
				srepr.push_str(format!("\tField: {}\n", fp.field));
				srepr.push_str(format!("\tReadable: {}\n", fp.readable.to_str()));
				srepr.push_str(format!("\tEditable: {}\n", fp.editable.to_str()));
			}
		}

		for op in self.objectPermissions.iter() {
			srepr.push_str("ObjectPermission: \n");
			srepr.push_str(format!("\tObject: {}\n", op.object));
			srepr.push_str(format!("\tAllowRead: {}\n", op.allowRead));
			srepr.push_str(format!("\tAllowCreate: {}\n", op.allowCreate));
			srepr.push_str(format!("\tAllowEdit: {}\n", op.allowEdit));
			srepr.push_str(format!("\tAllowDelete: {}\n", op.allowDelete));
			srepr.push_str(format!("\tViewAllRecords: {}\n", op.viewAllRecords));
			srepr.push_str(format!("\tModifyAllRecords: {}\n", op.modifyAllRecords));
		}

		for rt in self.recordTypeVisibilities.iter() {
			srepr.push_str("RecordTypeVisibility: \n");
			srepr.push_str(format!("\tRecordType: {}\n", rt.recordType));
			srepr.push_str(format!("\tDefault: {}\n", rt.default));
			srepr.push_str(format!("\tVisible: {}\n", rt.visible));
		}
		srepr
	}
}

#[deriving(Clone,Eq)]
pub struct FieldPermission {
	field : ~str,
	readable: bool,
	editable: bool
}

impl FieldPermission {
	pub fn new() -> FieldPermission {
		FieldPermission { 
			field: ~"", 
			readable: false, 
			editable: false 
		}
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
#[deriving(Clone,Eq)]
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
#[deriving(Clone,Eq)]
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