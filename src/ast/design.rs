use super::*;
use getset::*;

#[derive(Debug, Default, Getters, MutGetters)]
#[get = "pub"]
#[get_mut = "pub"]
pub struct Design {
    autoidx: usize,
	modules: Vec<Module>,
	attrs: HashMap<String, Const>,
}

impl Design {
	pub fn new() -> Self {
		Self::default()
	}
}