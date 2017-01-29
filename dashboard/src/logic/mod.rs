
mod user;
mod project;
mod like;
mod follow;
mod comment;

pub use self::user::User;
pub use self::project::Project;
pub use self::like::Like;
pub use self::follow::Follow;
pub use self::comment::Comment;

use Runtime;

pub trait LogicTrait<'a> {
    fn new(runtime: &'a Runtime) -> Self;

    fn run(&self);
}
