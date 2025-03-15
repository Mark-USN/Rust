
pub struct Post {
	state: Option<Box<dyn State>>,
	content: String,
}

impl Post {
	pub fn new() -> Post {
		Post {
			state: Some(Box::new(Draft {})),
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
		self.state.as_ref().unwrap().content(self)
	}

	// listing 17-15
	pub fn request_review(&mut self) {
		if let Some(s) = self.state.take() {
			self.state = Some(s.request_review())
		}
	}

	// listing 17-16
	pub fn approve(&mut self) {
		if let Some(s) = self.state.take() {
			self.state = Some(s.approve())
		}
	}

	pub fn reject(&mut self) {
		if let Some(s) = self.state.take() {
			self.state = Some(s.reject())
		}
	}


	pub fn state_name<'a>(&self) -> &'a str {
		if let Some(s) = self.state {
			s.state_name()
		}
		else {
			""
		}
	}


}

trait State {
	// listing 17-15 
	fn request_review(self: Box<Self>) -> Box<dyn State>;

	// listing 17-16
	fn approve(self: Box<Self>) -> Box<dyn State>;

	// listing 17-18
	fn content<'a>(&self, _post: &'a Post) -> &'a str {
		""
	}

	fn reject(self: Box<Self>) -> Box<dyn State>; 

	fn state_name<'a>(self: Box<Self>) -> &'a str;

}

struct Draft {}

impl State for Draft {
	// listing 17-15
	fn request_review(self: Box<Self>) -> Box<dyn State> {
		Box::new(PendingReview {})
	}

	// listing 17-16
	fn approve(self: Box<Self>) -> Box<dyn State> {
		self
	}

	fn reject(self: Box<Self>) -> Box<dyn State> {
		self
	}


	fn state_name<'a>(self: Box<Self>) -> & 'a str {
		"Draft"
	}

}

// listing 17-15
struct PendingReview{}

impl State for PendingReview {
	fn request_review(self: Box<Self>) -> Box<dyn State> {
		self
	}

	// listing 17-16
	fn approve(self: Box<Self>) -> Box<dyn State> {
		Box::new(Published {})
	}


	fn reject(self: Box<Self>) -> Box<dyn State> {
		Box::new(Draft {})
	}

	fn state_name<'a>(self: Box<Self>) -> &'a str {
		"PendingReview"
	}

}

// listing 17-16
struct Published {}

impl State for Published {
	fn request_review(self: Box<Self>) -> Box<dyn State> {
		self
	}

	// listing 17-16
	fn approve(self: Box<Self>) -> Box<dyn State> {
		self
	}

	// listing 17-18
	fn content<'a>(&self, _post: &'a Post) -> &'a str {
		&_post.content
	}

	fn reject(self: Box<Self>) -> Box<dyn State> {
		self
	}

	fn state_name<'a>(self: Box<Self>) -> &'a str {
		"Published"
	}

}
