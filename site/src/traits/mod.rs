use entities::{user::Model as UserModel, Id};

/// Trait that must be implemented by types that are rendered into templates
pub trait RenderData {
    fn user(&self) -> Option<UserModel>;
}

impl<'a, T> RenderData for &'a T
where
    T: RenderData,
{
    fn user(&self) -> Option<UserModel> {
        (*self).user()
    }
}
impl<'a, T> RenderData for &'a mut T
where
    T: RenderData,
{
    fn user(&self) -> Option<UserModel> {
        (*self).user()
    }
}
