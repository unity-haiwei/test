
mod user;
mod project;
mod like;
mod follow;
mod comment;
mod job;
mod proposal;
mod team;
mod message;

pub use self::user::User;
pub use self::project::Project;
pub use self::like::Like;
pub use self::follow::Follow;
pub use self::comment::Comment;
pub use self::job::Job;
pub use self::proposal::Proposal;
pub use self::team::Team;
pub use self::message::Message;

use Runtime;

pub trait LogicTrait<'a> {
    fn new(runtime: &'a Runtime) -> Self;

    fn run(&self);
}
