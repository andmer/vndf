use protocol_ng::{
	Action,
	Step,
};


pub struct ActionAssembler {
	next_seq : u64,
	added    : Vec<Step>,
	assembled: Option<Action>,
}

impl ActionAssembler {
	pub fn new() -> ActionAssembler {
		ActionAssembler {
			next_seq : 0,
			added    : Vec::new(),
			assembled: None,
		}
	}

	pub fn add_step(&mut self, step: Step) {
		self.added.push(step);
	}

	pub fn assemble(&mut self) -> Action {
		let action = match self.assembled {
			Some(_) =>
				self.assembled.take().unwrap(),
			None => {
				let action = Action {
					seq  : self.next_seq,
					steps: self.added.clone(),
				};

				self.next_seq += 1;
				self.added.clear();

				action
			},
		};

		self.assembled = Some(action.clone());
		action
	}

	pub fn process_receipt(&mut self, seq: u64) {
		let is_confirmed = match self.assembled {
			Some(ref action) => seq >= action.seq,
			None             => false,
		};

		if is_confirmed {
			self.assembled = None;
		}
	}
}