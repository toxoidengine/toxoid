pub struct App {
	pub world: i32
}

impl App {
	pub fn new() -> App {
			App::default()
	}

	pub fn default() -> App {
		App {
			world: 42
		}
	}
}
