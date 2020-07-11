use crate::domain::post::entity::Post;

pub trait PostRepository {
    fn all(&self) -> Vec<Post>;
}
