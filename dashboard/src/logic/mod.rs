
mod project;
mod user;
mod like;
mod follow;
mod comment;

pub use self::project::Project;
pub use self::user::User;
pub use self::like::Like;
pub use self::follow::Follow;
pub use self::comment::Comment;


pub trait LogicTrait {
    fn new() -> Self;

    fn run(&self);
}
