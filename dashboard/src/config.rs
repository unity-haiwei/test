
pub static MONGO_URI: &'static str = "mongodb://localhost:27017/";
pub static MONGO_IP: &'static str = "localhost";
pub const MONGO_PORT: u16 = 27017;
pub static DB_NAME: &'static str = "mp";

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

pub const COMMAND_TOTAL_JOB: &'static str = "total_job";
pub const COMMAND_NEW_JOB: &'static str = "new_job";
pub const COMMAND_TOTAL_JOB_USER_PERCENT: &'static str = "total_job_user_percent";
pub const COMMAND_DAILY_JOB_USER_PERCENT: &'static str = "daily_job_user_percent";

pub const COMMAND_TOTAL_TASK: &'static str = "total_task";
pub const COMMAND_NEW_TASK: &'static str = "new_task";
pub const COMMAND_TOTAL_TASK_USER_PERCENT: &'static str = "total_task_user_percent";
pub const COMMAND_DAILY_TASK_USER_PERCENT: &'static str = "daily_task_user_percent";

pub const COMMAND_TOTAL_APPLICATION: &'static str = "total_application";
pub const COMMAND_NEW_APPLICATION: &'static str = "new_application";
pub const COMMAND_TOTAL_APPLICATION_USER_PERCENT: &'static str = "total_application_user_percent";
pub const COMMAND_DAILY_APPLICATION_USER_PERCENT: &'static str = "daily_application_user_percent";

pub const COMMAND_TOTAL_INVITED: &'static str = "total_invitation";
pub const COMMAND_NEW_INVITED: &'static str = "new_invitation";
pub const COMMAND_TOTAL_INVITED_USER_PERCENT: &'static str = "total_invitation_user_percent";
pub const COMMAND_DAILY_INVITED_USER_PERCENT: &'static str = "daily_invitation_user_percent";

pub const COMMAND_TOTAL_MATCH: &'static str = "total_match";
pub const COMMAND_NEW_MATCH: &'static str = "new_match";
pub const COMMAND_TOTAL_MATCH_USER_PERCENT: &'static str = "total_match_user_percent";
pub const COMMAND_DAILY_MATCH_USER_PERCENT: &'static str = "daily_match_user_percent";

pub const COMMAND_TOTAL_TEAM: &'static str = "total_company";
pub const COMMAND_NEW_TEAM: &'static str = "new_company";
pub const COMMAND_TOTAL_TEAM_USER_PERCENT: &'static str = "total_company_user_percent";
pub const COMMAND_DAILY_TEAM_USER_PERCENT: &'static str = "daily_company_user_percent";

pub const COMMAND_TOTAL_MESSAGE: &'static str = "total_message";
pub const COMMAND_NEW_MESSAGE: &'static str = "new_message";
pub const COMMAND_TOTAL_MESSAGE_USER_PERCENT: &'static str = "total_message_user_percent";
pub const COMMAND_DAILY_MESSAGE_USER_PERCENT: &'static str = "daily_message_user_percent";



pub static ITEM_TYPE_PROJECT: &'static str = "project";
pub static ITEM_TYPE_JOB: &'static str = "job";
pub static ITEM_TYPE_USER: &'static str = "user";
