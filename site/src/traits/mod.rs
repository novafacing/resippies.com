use entities::{theme::Theme, user::Model as UserModel, Id};

/// Trait that must be implemented by types that are rendered into templates
pub trait RenderData {
    fn user(&self) -> Option<UserModel>;
    fn theme(&self) -> Theme;
}

impl<'a, T> RenderData for &'a T
where
    T: RenderData,
{
    fn user(&self) -> Option<UserModel> {
        (*self).user()
    }

    fn theme(&self) -> Theme {
        (*self).theme()
    }
}
