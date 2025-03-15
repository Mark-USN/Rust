pub struct Post {
	mode: Mode,
	content: String,
}

impl Post {

	pub fn new() -> Post {
		Post {
			mode: Mode::Draft,
			content: String::new(),
		}
	}

	// listing 17-13
	pub fn add_text(&mut self, text: &str) {
		self.content.push_str(text);
	}

	// listing 17-14
	pub fn content(&self) -> &str {
		// listing 17-17
		self.mode.content(&self)
	}
	
	// listing 17-15
	pub fn request_review(&mut self) {
		self.mode = self.mode.request_review();
	}

	pub fn approve(&mut self) {
		self.mode = self.mode.approve();
	}

}

trait State {

	fn request_review(&self) -> Mode;

	fn approve(&self) -> Mode;

	// listing 17-18
	fn content<'a>(&self, post: &'a Post) -> &'a str;
}

// 20241127 MMH Change to enum

enum Mode {
	Draft,
	PendingReview,
	Published,
}

impl State for Mode {
	fn request_review(&self) -> Mode {
		match self {
			Mode::Draft => Mode::PendingReview,
			//_ => *self,
			Mode::PendingReview => Mode::PendingReview, 
			Mode::Published => Mode::Published,
			//Mode::PendingReview => self, 
			//Mode::Published => self,
		}
	}

	fn approve(&self) -> Mode {
		match self {
			Mode::Draft => Mode::Draft,
			Mode::PendingReview => Mode::Published, 
			Mode::Published => Mode::Published,

		}
	}

	fn content<'a>(&self, post: &'a Post) -> &'a str {
		match self {
			Mode::Draft => "",
			Mode::PendingReview => "",
			Mode::Published => {
				&post.content
			}
		}
	}

}


