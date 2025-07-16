use chrono::Local;

#[derive(Debug, Clone)]
pub struct Schedule {
    pub id: String,
    pub command: String,
    pub scheduled_time: Option<String>, // "2024-01-01T10:00" 形式
    pub _memo: String,
    pub created_at: String,
    pub status: ScheduleStatus,
    pub is_shell_mode: bool, // シェルモード実行フラグ
    pub branch: String,      // git worktreeのbranch
    pub execution_path: String, // 実行ディレクトリパス
}

#[derive(Debug, Clone, PartialEq)]
pub enum ScheduleStatus {
    Pending,
    Completed,
    Failed,
}

#[derive(Debug, Clone)]
pub struct ExecutionHistory {
    pub id: String,
    pub command: String,
    pub executed_at: String,
    pub execution_type: ExecutionType,
    pub status: ExecutionStatus,
    pub output: String,
    pub branch: String, // git worktreeのbranch
    pub execution_path: String, // 実行ディレクトリパス
}

#[derive(Debug, Clone, PartialEq)]
pub enum ExecutionType {
    Manual,
    #[allow(dead_code)]
    Auto,
    FromSchedule,
    ShellMode,
}

#[derive(Debug, Clone, PartialEq)]
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
