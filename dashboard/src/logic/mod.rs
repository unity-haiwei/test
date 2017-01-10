
mod project;
mod user;

pub use self::project::Project;
pub use self::user::User;

pub trait LogicTrait {
    fn run(&self);
}