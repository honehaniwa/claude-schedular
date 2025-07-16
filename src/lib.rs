#[cfg(feature = "gui")]
pub mod components;
pub mod git;
pub mod models;
pub mod utils;
pub mod cli;
pub mod cli_commands;
pub mod cli_handlers;
pub mod config;
pub mod database;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_utils_current_datetime() {
        let datetime = utils::get_current_datetime();
        assert!(datetime.contains("T"));
        assert!(datetime.len() >= 16); // YYYY-MM-DDTHH:MM
    }

    #[test]
    fn test_models_schedule_status() {
        use models::ScheduleStatus;

        assert_eq!(ScheduleStatus::Pending.to_string(), "待機中");
        assert_eq!(ScheduleStatus::Completed.to_string(), "完了");
        assert_eq!(ScheduleStatus::Failed.to_string(), "失敗");
    }

    #[test]
    fn test_models_execution_status() {
        use models::ExecutionStatus;

        assert_eq!(ExecutionStatus::Success.to_string(), "成功");
        assert_eq!(ExecutionStatus::Failed.to_string(), "失敗");
    }

    #[test]
    fn test_git_current_branch() {
        let branch = git::get_current_branch();
        assert!(!branch.is_empty());
    }

    #[test]
    fn test_utils_time_reached() {
        use chrono::Local;

        // 過去の時刻をテスト
        let past_time = Local::now()
            .checked_sub_signed(chrono::Duration::hours(1))
            .unwrap()
            .format("%Y-%m-%dT%H:%M")
            .to_string();

        assert!(utils::is_time_reached(&past_time));

        // 未来の時刻をテスト
        let future_time = Local::now()
            .checked_add_signed(chrono::Duration::hours(1))
            .unwrap()
            .format("%Y-%m-%dT%H:%M")
            .to_string();

        assert!(!utils::is_time_reached(&future_time));
    }

    #[test]
    fn test_utils_build_scheduled_time() {
        let scheduled_time = utils::build_scheduled_time(false, 12, 30);
        assert!(scheduled_time.contains("T12:30"));
    }
}
