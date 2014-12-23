use acpe::protocol::{
	ActionHeader,
	Encoder,
	Seq,
};

use common::protocol::Step;


pub struct ActionAssembler {
	next_seq : Seq,
	added    : Vec<Step>,
	assembled: Option<Vec<u8>>,
}

impl<'a> ActionAssembler {
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

	pub fn assemble(&mut self, encoder: &mut Encoder) -> Vec<u8> {
		match self.assembled {
			Some(ref message) => return message.clone(),
			None              => (),
		}

		let mut action = encoder.message(&ActionHeader { id: self.next_seq });

		let mut is_first_step = true;
		loop {
			let step = match self.added.remove(0) {
				Some(step) => step,
				None       => break,
			};

			if !action.add(&step) {
				if is_first_step {
					panic!(
						"Failed to add first step of an action. Since the \
						action is still empty when adding the first step, this \
						means the step is too large to ever be added to an \
						action. This is a bug, as such a step should have been \
						rejected when it was created."
					);
				}
				self.added.insert(0, step);
				break;
			}

			is_first_step = false;
		}

		let message = action.encode();

		let mut assembled = Vec::new();
		assembled.push_all(message);

		self.assembled = Some(assembled.clone());
		assembled
	}

	pub fn process_receipt(&mut self, seq: Seq) {
		let is_confirmed = match self.assembled {
			Some(_) => seq >= self.next_seq,
			None    => false,
		};

		if is_confirmed {
			self.assembled = None;
			self.next_seq += 1;
		}
	}
}
