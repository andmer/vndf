use std::from_str;
use std::intrinsics::TypeId;

use physics::{Body, Radians, Vec2};


#[deriving(Show)]
pub enum Message {
	SelfInfo(SelfInfo),
	Create(Create),
	Update(Update),
	Remove(Remove),
	Invalid(~str)
}

impl Message {
	pub fn from_str(s: &str) -> Message {
		let words: ~[&str] = s.words().collect();

		match words[0] {
			"SELF_ID" => SelfInfo(SelfInfo::from_words(words)),
			"CREATE"  => Create(Create::from_words(words)),
			"UPDATE"  => Update(Update::from_words(words)),
			"REMOVE"  => Remove(Remove::from_words(words)),
			_         => Invalid(s.to_owned())
		}
	}

	pub fn type_id(&self) -> TypeId {
		match *self {
			SelfInfo(_) => TypeId::of::<SelfInfo>(),
			Create(_)   => TypeId::of::<Create>(),
			Update(_)   => TypeId::of::<Update>(),
			Remove(_)   => TypeId::of::<Remove>(),
			Invalid(_)  => TypeId::of::<()>()
		}
	}
}


#[deriving(Show)]
pub struct SelfInfo {
	pub id: uint
}

impl SelfInfo {
	pub fn from_words(words: ~[&str]) -> SelfInfo {
		let id: Option<uint> = from_str::from_str(words[1]);

		SelfInfo {
			id: id.unwrap()
		}
	}

	pub fn to_str(&self) -> ~str {
		format!(
			"SELF_ID {}",
			self.id)
	}
}


#[deriving(Show)]
pub struct Create {
	pub id  : uint,
	pub kind: ~str
}

impl Create {
	pub fn from_words(words: ~[&str]) -> Create {
		let id  : Option<uint> = from_str::from_str(words[1]);
		let kind               = words[2].to_owned();

		Create {
			id  : id.unwrap(),
			kind: kind
		}
	}

	pub fn to_str(&self) -> ~str {
		format!(
			"CREATE {} {}",
			self.id,
			self.kind)
	}
}


#[deriving(Eq, Show)]
pub struct Update {
	pub id  : uint,
	pub body: Body
}

impl Update {
	pub fn from_words(words: ~[&str]) -> Update {
		let id: Option<uint> = from_str::from_str(words[1]);
		let x : Option<f64>  = from_str::from_str(words[2]);
		let y : Option<f64>  = from_str::from_str(words[3]);

		Update {
			id  : id.unwrap(),
			body: Body {
				position: Vec2 {
					x: x.unwrap(),
					y: y.unwrap()
				},
				velocity: Vec2 {
					x: 0.0,
					y: 0.0
				},
				attitude: Radians(0.0)
			}
		}
	}
	pub fn to_str(&self) -> ~str {
		format!(
			"UPDATE {} {} {} {} {} {}",
			self.id,
			self.body.position.x,
			self.body.position.y,
			self.body.velocity.x,
			self.body.velocity.y,
			self.body.attitude)
	}
}


#[deriving(Show)]
pub struct Remove {
	pub id: uint
}

impl Remove {
	pub fn from_words(words: ~[&str]) -> Remove {
		let id: Option<uint> = from_str::from_str(words[1]);

		Remove {
			id: id.unwrap()
		}
	}

	pub fn to_str(&self) -> ~str {
		format!(
			"REMOVE {}",
			self.id)
	}
}
