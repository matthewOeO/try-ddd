use crate::domain::post::entity::Post;
use crate::domain::post::repository::PostRepository;

use std::collections::HashMap;
use std::cell::RefCell;

pub struct InMemoryPostRepository {
    posts: RefCell<HashMap<String, Post>>
}

impl InMemoryPostRepository {
    pub fn new() -> InMemoryPostRepository {
        let mut posts: HashMap<String, Post> = HashMap::new();
        return InMemoryPostRepository {
            posts: RefCell::new(posts)
        }
    }
}

impl PostRepository for InMemoryPostRepository {
    fn all(&self) -> Vec<Post> {
        let mut ret = Vec::new();
        for v in self.posts.borrow().values() {
            ret.push(v.clone())
        }
        ret
    }
}
