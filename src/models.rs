use chrono::Local;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Schedule {
    pub id: String,
    pub command: String,
    pub scheduled_time: Option<String>, // "2024-01-01T10:00" 形式
    pub _memo: String,
    pub created_at: String,
    pub status: ScheduleStatus,
    pub is_shell_mode: bool,    // シェルモード実行フラグ
    pub branch: String,         // git worktreeのbranch
    pub execution_path: String, // 実行ディレクトリパス
    #[serde(default)]
    pub claude_skip_permissions: bool, // --dangerously-skip-permissions フラグ
    #[serde(default)]
    pub claude_continue_from_last: bool, // -c フラグ
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum ScheduleStatus {
    Pending,
    Completed,
    Failed,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExecutionHistory {
    pub id: String,
    pub command: String,
    pub executed_at: String,
    pub execution_type: ExecutionType,
    pub status: ExecutionStatus,
    pub output: String,
    pub branch: String,         // git worktreeのbranch
    pub execution_path: String, // 実行ディレクトリパス
    #[serde(default)]
    pub claude_skip_permissions: bool, // --dangerously-skip-permissions フラグ
    #[serde(default)]
    pub claude_continue_from_last: bool, // -c フラグ
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum ExecutionType {
    Manual,
    #[allow(dead_code)]
    Auto,
    FromSchedule,
    ShellMode,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum ExecutionStatus {
    Success,
    Failed,
}

impl Default for Schedule {
    fn default() -> Self {
        Self {
            id: format!("schedule_{}", chrono::Utc::now().timestamp()),
            command: String::new(),
            scheduled_time: None,
            _memo: String::new(),
            created_at: Local::now().format("%Y-%m-%d %H:%M:%S").to_string(),
            status: ScheduleStatus::Pending,
            is_shell_mode: false,
            branch: "main".to_string(),
            execution_path: std::env::current_dir()
                .unwrap_or_else(|_| std::path::PathBuf::from("."))
                .to_string_lossy()
                .to_string(),
            claude_skip_permissions: false,
            claude_continue_from_last: false,
        }
    }
}

impl std::fmt::Display for ScheduleStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ScheduleStatus::Pending => write!(f, "待機中"),
            ScheduleStatus::Completed => write!(f, "完了"),
            ScheduleStatus::Failed => write!(f, "失敗"),
        }
    }
}

impl std::fmt::Display for ExecutionType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ExecutionType::Manual => write!(f, "手動実行"),
            ExecutionType::Auto => write!(f, "自動実行"),
            ExecutionType::FromSchedule => write!(f, "スケジュール実行"),
            ExecutionType::ShellMode => write!(f, "シェル実行"),
        }
    }
}

impl std::fmt::Display for ExecutionStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ExecutionStatus::Success => write!(f, "成功"),
            ExecutionStatus::Failed => write!(f, "失敗"),
        }
    }
}

// Helper methods for database
impl ScheduleStatus {
    pub fn to_string(&self) -> String {
        match self {
            ScheduleStatus::Pending => "pending".to_string(),
            ScheduleStatus::Completed => "completed".to_string(),
            ScheduleStatus::Failed => "failed".to_string(),
        }
    }

    pub fn from_string(s: &str) -> Self {
        match s.to_lowercase().as_str() {
            "pending" => ScheduleStatus::Pending,
            "completed" => ScheduleStatus::Completed,
            "failed" => ScheduleStatus::Failed,
            _ => ScheduleStatus::Pending,
        }
    }
}

impl ExecutionType {
    pub fn to_string(&self) -> String {
        match self {
            ExecutionType::Manual => "manual".to_string(),
            ExecutionType::Auto => "auto".to_string(),
            ExecutionType::FromSchedule => "from_schedule".to_string(),
            ExecutionType::ShellMode => "shell_mode".to_string(),
        }
    }

    pub fn from_string(s: &str) -> Self {
        match s.to_lowercase().as_str() {
            "manual" => ExecutionType::Manual,
            "auto" => ExecutionType::Auto,
            "from_schedule" => ExecutionType::FromSchedule,
            "shell_mode" => ExecutionType::ShellMode,
            _ => ExecutionType::Manual,
        }
    }
}

impl ExecutionStatus {
    pub fn to_string(&self) -> String {
        match self {
            ExecutionStatus::Success => "success".to_string(),
            ExecutionStatus::Failed => "failed".to_string(),
        }
    }

    pub fn from_string(s: &str) -> Self {
        match s.to_lowercase().as_str() {
            "success" => ExecutionStatus::Success,
            "failed" => ExecutionStatus::Failed,
            _ => ExecutionStatus::Failed,
        }
    }
}
