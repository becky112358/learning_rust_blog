pub struct DraftPost {
    content: String,
}

pub struct PendingFirstReviewPost {
    content: String,
}

pub struct PendingSecondReviewPost {
    content: String,
}

pub struct Post {
    content: String,
}

impl DraftPost {
    pub fn add_text(&mut self, text: &str) {
        self.content.push_str(text);
    }

    pub fn request_review(self) -> PendingFirstReviewPost {
        PendingFirstReviewPost {
            content: self.content,
        }
    }
}

impl PendingFirstReviewPost {
    pub fn reject(self) -> DraftPost {
        DraftPost {
            content: self.content,
        }
    }

    pub fn approve(self) -> PendingSecondReviewPost {
        PendingSecondReviewPost {
            content: self.content,
        }
    }
}

impl PendingSecondReviewPost {
    pub fn reject(self) -> DraftPost {
        DraftPost {
            content: self.content,
        }
    }

    pub fn approve(self) -> Post {
        Post {
            content: self.content,
        }
    }
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

