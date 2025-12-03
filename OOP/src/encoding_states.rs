// This whole file shows an implementation of the state pattern using different structs for each state
// This approach uses the type system to enforce the state transitions at compile time
// A Post can only be created in the Draft state, and can only transition to PendingReview and then to Published
// There is no way to have a Post in an invalid state or skip states




fn run() {
    let mut post = blog::Post::new();

    post.add_text("I ate a salad for lunch today");

    let post = post.request_review();

    let post = post.approve();

    assert_eq!("I ate a salad for lunch today", post.content());
}


// With this implementation we can only create a Post in the Draft state making it impossible
// to have a Post that is published without going through the proper steps
// The only way to add text to the post is through the DraftPost struct
mod blog{
    pub struct Post {
        content: String,
    }

    pub struct DraftPost {
        content: String,
    }

    impl Post {
        // Creates a new Post of type DraftPost with an empty content
        pub fn new() -> DraftPost {
            // This is a constructor method, like saying new DraftPost in other languages
            DraftPost {
                content: String::new(),
            }
        }

        // Returns the content of the Post from the self reference we passed
        pub fn content(&self) -> &str {
            &self.content
        }
    }

    impl DraftPost {
        // Only public method to add text to the DraftPost type
        pub fn add_text(&mut self, text: &str) {
            // Appends the provided text to the content field, we need mut self ref to modify it
            self.content.push_str(text);
        }

        pub fn request_review(self) -> PendingReviewPost {
            PendingReviewPost {
                content: self.content,
            }
        }
    }

    pub struct PendingReviewPost {
        content: String,
    }

    // Here we consume the PendingReviewPost, meaning we transfer ownership of it
    // so PendingReviewPost is no longer valid after this method is called and we get a Post back
    // which we can use to access the content
    impl PendingReviewPost {
        pub fn approve(self) -> Post {
            Post {
                // This is valid because we are consuming self, so we can move the content out
                content: self.content,
            }
        }
    }
}