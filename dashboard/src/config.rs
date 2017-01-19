
pub static MONGO_URI: &'static str = "mongodb://localhost:27017/";
pub static MONGO_IP: &'static str = "localhost";
pub const MONGO_PORT: u16 = 27017;
pub static DB_NAME: &'static str = "mp";
pub static SCRIPTS_PATH: &'static str = "/home/haiwei/work/genesis-backend/dashboard/scripts/marketplace";

pub const COMMAND_DROP: &'static str = "drop";
pub const COMMAND_SETUP: &'static str = "setup";

pub const COMMAND_TOTAL_PROJECT: &'static str = "total_project";
pub const COMMAND_NEW_PROJECT: &'static str = "new_project";
pub const COMMAND_TOTAL_PROJECT_USER_PERCENT: &'static str = "total_project_user_percent";
pub const COMMAND_DAILY_PROJECT_USER_PERCENT: &'static str = "daily_project_user_percent";

pub const COMMAND_ZEROTH_DAY_COMPLETENESS: &'static str = "zeroth_day_completeness";
pub const COMMAND_THIRD_DAY_COMPLETENESS: &'static str = "third_day_completeness";
pub const COMMAND_SEVENTH_DAY_COMPLETENESS: &'static str = "seventh_day_completeness";

pub const COMMAND_TOTAL_LIKE: &'static str = "total_like";
pub const COMMAND_NEW_LIKE: &'static str = "new_like";
pub const COMMAND_TOTAL_LIKE_USER_PERCENT: &'static str = "total_like_user_percent";
pub const COMMAND_DAILY_LIKE_USER_PERCENT: &'static str = "daily_like_user_percent";

pub const COMMAND_TOTAL_FOLLOW: &'static str = "total_follow";
pub const COMMAND_NEW_FOLLOW: &'static str = "new_follow";
pub const COMMAND_TOTAL_FOLLOW_USER_PERCENT: &'static str = "total_follow_user_percent";
pub const COMMAND_DAILY_FOLLOW_USER_PERCENT: &'static str = "daily_follow_user_percent";

pub const COMMAND_TOTAL_COMMENT: &'static str = "total_comment";
pub const COMMAND_NEW_COMMENT: &'static str = "new_comment";
pub const COMMAND_TOTAL_COMMENT_USER_PERCENT: &'static str = "total_comment_user_percent";
pub const COMMAND_DAILY_COMMENT_USER_PERCENT: &'static str = "daily_comment_user_percent";

pub static ITEM_TYPE_PROJECT: &'static str = "project";
pub static ITEM_TYPE_JOB: &'static str = "job";
pub static ITEM_TYPE_USER: &'static str = "user";
