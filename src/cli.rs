use chrono::NaiveDate;
use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(name = "claude-scheduler")]
#[command(author, version, about = "Claude Scheduler: Claude AI command scheduler with Git worktree support", long_about = None)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,

    /// Enable verbose output
    #[arg(short, long, global = true)]
    pub verbose: bool,
}

#[derive(Subcommand)]
pub enum Commands {
    /// Execute a command immediately
    Exec {
        /// Command to execute
        command: String,

        /// Execution mode [claude|shell]
        #[arg(short, long, default_value = "claude")]
        mode: String,

        /// Git worktree branch
        #[arg(short, long)]
        branch: Option<String>,

        /// Enable Git worktree parallel execution
        #[arg(short, long)]
        worktree: bool,
    },

    /// Schedule a command for later execution
    Schedule {
        /// Command to schedule
        command: String,

        /// Execution time (HH:MM format)
        #[arg(short, long)]
        time: String,

        /// Execution date [today|tomorrow|YYYY-MM-DD]
        #[arg(short, long, default_value = "today")]
        date: String,

        /// Execution mode [claude|shell]
        #[arg(short, long, default_value = "claude")]
        mode: String,

        /// Git worktree branch
        #[arg(short, long)]
        branch: Option<String>,

        /// Enable Git worktree parallel execution
        #[arg(short, long)]
        worktree: bool,

        /// Add a memo
        #[arg(long)]
        memo: Option<String>,
    },

    /// List scheduled commands
    List {
        /// Filter by status [pending|completed|failed]
        #[arg(short, long)]
        status: Option<String>,

        /// Output format [table|json|csv]
        #[arg(short, long, default_value = "table")]
        format: String,

        /// Limit number of results
        #[arg(short = 'n', long)]
        limit: Option<usize>,
    },

    /// Show execution history
    History {
        /// Filter by status [success|failed]
        #[arg(short, long)]
        status: Option<String>,

        /// Filter by execution type [manual|auto|shell]
        #[arg(short = 't', long = "type")]
        exec_type: Option<String>,

        /// Filter by branch
        #[arg(short, long)]
        branch: Option<String>,

        /// Output format [table|json|csv]
        #[arg(short, long, default_value = "table")]
        format: String,

        /// Limit number of results
        #[arg(short = 'n', long)]
        limit: Option<usize>,

        /// Start date (YYYY-MM-DD)
        #[arg(long)]
        from: Option<NaiveDate>,

        /// End date (YYYY-MM-DD)
        #[arg(long)]
        to: Option<NaiveDate>,
    },

    /// Run as a daemon process
    Daemon {
        /// API port number
        #[arg(short, long, default_value = "8080")]
        port: u16,

        /// Check interval in seconds
        #[arg(short, long, default_value = "5")]
        interval: u64,

        /// PID file path
        #[arg(long)]
        pid_file: Option<String>,

        /// Log file path
        #[arg(long)]
        log_file: Option<String>,

        /// Run in background
        #[arg(short, long)]
        detach: bool,
    },

    /// Manage configuration
    Config {
        #[command(subcommand)]
        action: ConfigAction,
    },
}

#[derive(Subcommand)]
pub enum ConfigAction {
    /// Show current configuration
    Show,

    /// Set a configuration value
    Set {
        /// Configuration key
        key: String,

        /// Configuration value
        value: String,
    },

    /// Get a configuration value
    Get {
        /// Configuration key
        key: String,
    },
}
