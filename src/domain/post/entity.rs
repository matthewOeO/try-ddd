#[derive(Debug, Clone)]
pub struct Post {
    id: u64,
    title: String
}

impl Post {
    pub fn new(id: u64, title: String) -> Post {
        Post {
            id,
            title
        }
    }
}
