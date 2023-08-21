pub struct Post {
    content: String,
}

pub struct DraftPost {
    content: String,
}

pub struct PendingTwoReviewPost {
    content: String,
}

pub struct PendingReviewPost {
    content: String,
}

impl Post {
    pub fn new() -> DraftPost {
        DraftPost {
            content: String::new(),
        }
    }

    pub fn content(&self) -> &str {
        &self.content
    }
}

impl DraftPost {
    pub fn add_text(&mut self, text: &str) {
        self.content.push_str(text);
    }

    pub fn request_review(self) -> PendingTwoReviewPost {
        PendingTwoReviewPost {
            content: self.content,
        }
    }
}

impl PendingReviewPost {
    pub fn approve(self) -> Post {
        Post {
            content: self.content,
        }
    }
}

impl PendingTwoReviewPost {
    pub fn approve(self) -> PendingReviewPost {
        PendingReviewPost {
            content: self.content,
        }
    }
}
