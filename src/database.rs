use anyhow::Result;
use chrono::{DateTime, Local};
use sqlx::sqlite::SqlitePool;
use std::path::Path;

use crate::models::{ExecutionHistory, ExecutionStatus, ExecutionType, Schedule, ScheduleStatus};

pub struct Database {
    pool: SqlitePool,
}

impl Database {
    pub async fn new(path: &Path) -> Result<Self> {
        // Create parent directory if it doesn't exist
        if let Some(parent) = path.parent() {
            std::fs::create_dir_all(parent)?;
        }

        let database_url = format!("sqlite:{}", path.display());
        let pool = SqlitePool::connect(&database_url).await?;

        let db = Self { pool };
        db.initialize().await?;
        Ok(db)
    }

    async fn initialize(&self) -> Result<()> {
        // Create schedules table
        sqlx::query(
            r#"
            CREATE TABLE IF NOT EXISTS schedules (
                id TEXT PRIMARY KEY,
                command TEXT NOT NULL,
                scheduled_time TEXT,
                memo TEXT,
                created_at TEXT NOT NULL,
                status TEXT NOT NULL,
                is_shell_mode INTEGER NOT NULL,
                branch TEXT NOT NULL,
                execution_path TEXT NOT NULL DEFAULT '.'
            )
            "#,
        )
        .execute(&self.pool)
        .await?;

        // Create execution_history table
        sqlx::query(
            r#"
            CREATE TABLE IF NOT EXISTS execution_history (
                id TEXT PRIMARY KEY,
                command TEXT NOT NULL,
                executed_at TEXT NOT NULL,
                execution_type TEXT NOT NULL,
                status TEXT NOT NULL,
                output TEXT NOT NULL,
                branch TEXT NOT NULL,
                execution_path TEXT NOT NULL DEFAULT '.'
            )
            "#,
        )
        .execute(&self.pool)
        .await?;

        // Create configuration table
        sqlx::query(
            r#"
            CREATE TABLE IF NOT EXISTS configuration (
                key TEXT PRIMARY KEY,
                value TEXT NOT NULL
            )
            "#,
        )
        .execute(&self.pool)
        .await?;

        Ok(())
    }

    // Schedule methods
    pub async fn create_schedule(&self, schedule: &Schedule) -> Result<()> {
        sqlx::query(
            r#"
            INSERT INTO schedules (id, command, scheduled_time, memo, created_at, status, is_shell_mode, branch, execution_path)
            VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?)
            "#,
        )
        .bind(&schedule.id)
        .bind(&schedule.command)
        .bind(&schedule.scheduled_time)
        .bind(&schedule._memo)
        .bind(&schedule.created_at)
        .bind(schedule.status.to_string())
        .bind(schedule.is_shell_mode as i32)
        .bind(&schedule.branch)
        .bind(&schedule.execution_path)
        .execute(&self.pool)
        .await?;

        Ok(())
    }

    pub async fn get_schedules(
        &self,
        status_filter: Option<ScheduleStatus>,
        limit: Option<usize>,
    ) -> Result<Vec<Schedule>> {
        let mut query = "SELECT * FROM schedules".to_string();

        if let Some(status) = status_filter {
            query.push_str(&format!(" WHERE status = '{}'", status.to_string()));
        }

        query.push_str(" ORDER BY created_at DESC");

        if let Some(limit) = limit {
            query.push_str(&format!(" LIMIT {}", limit));
        }

        let rows = sqlx::query(&query).fetch_all(&self.pool).await?;

        let schedules = rows
            .into_iter()
            .map(|row| Schedule {
                id: sqlx::Row::get(&row, "id"),
                command: sqlx::Row::get(&row, "command"),
                scheduled_time: sqlx::Row::get(&row, "scheduled_time"),
                _memo: sqlx::Row::get(&row, "memo"),
                created_at: sqlx::Row::get(&row, "created_at"),
                status: ScheduleStatus::from_string(&sqlx::Row::get::<String, _>(&row, "status")),
                is_shell_mode: sqlx::Row::get::<i32, _>(&row, "is_shell_mode") != 0,
                branch: sqlx::Row::get(&row, "branch"),
                execution_path: std::env::current_dir()
                    .unwrap_or_else(|_| std::path::PathBuf::from("."))
                    .to_string_lossy()
                    .to_string(),
            })
            .collect();

        Ok(schedules)
    }

    pub async fn update_schedule_status(&self, id: &str, status: ScheduleStatus) -> Result<()> {
        sqlx::query(
            r#"
            UPDATE schedules SET status = ? WHERE id = ?
            "#,
        )
        .bind(status.to_string())
        .bind(id)
        .execute(&self.pool)
        .await?;

        Ok(())
    }

    // Execution history methods
    pub async fn create_execution_history(&self, history: &ExecutionHistory) -> Result<()> {
        sqlx::query(
            r#"
            INSERT INTO execution_history (id, command, executed_at, execution_type, status, output, branch, execution_path)
            VALUES (?, ?, ?, ?, ?, ?, ?, ?)
            "#,
        )
        .bind(&history.id)
        .bind(&history.command)
        .bind(&history.executed_at)
        .bind(history.execution_type.to_string())
        .bind(history.status.to_string())
        .bind(&history.output)
        .bind(&history.branch)
        .bind(&history.execution_path)
        .execute(&self.pool)
        .await?;

        Ok(())
    }

    pub async fn get_execution_history(
        &self,
        status_filter: Option<ExecutionStatus>,
        type_filter: Option<ExecutionType>,
        branch_filter: Option<String>,
        from_date: Option<DateTime<Local>>,
        to_date: Option<DateTime<Local>>,
        limit: Option<usize>,
    ) -> Result<Vec<ExecutionHistory>> {
        let mut query = "SELECT * FROM execution_history WHERE 1=1".to_string();

        if let Some(status) = status_filter {
            query.push_str(&format!(" AND status = '{}'", status.to_string()));
        }

        if let Some(exec_type) = type_filter {
            query.push_str(&format!(
                " AND execution_type = '{}'",
                exec_type.to_string()
            ));
        }

        if let Some(branch) = branch_filter {
            query.push_str(&format!(" AND branch = '{}'", branch));
        }

        if let Some(from) = from_date {
            query.push_str(&format!(
                " AND executed_at >= '{}'",
                from.format("%Y-%m-%d %H:%M:%S")
            ));
        }

        if let Some(to) = to_date {
            query.push_str(&format!(
                " AND executed_at <= '{}'",
                to.format("%Y-%m-%d %H:%M:%S")
            ));
        }

        query.push_str(" ORDER BY executed_at DESC");

        if let Some(limit) = limit {
            query.push_str(&format!(" LIMIT {}", limit));
        }

        let rows = sqlx::query(&query).fetch_all(&self.pool).await?;

        let history = rows
            .into_iter()
            .map(|row| ExecutionHistory {
                id: sqlx::Row::get(&row, "id"),
                command: sqlx::Row::get(&row, "command"),
                executed_at: sqlx::Row::get(&row, "executed_at"),
                execution_type: ExecutionType::from_string(&sqlx::Row::get::<String, _>(
                    &row,
                    "execution_type",
                )),
                status: ExecutionStatus::from_string(&sqlx::Row::get::<String, _>(&row, "status")),
                output: sqlx::Row::get(&row, "output"),
                branch: sqlx::Row::get(&row, "branch"),
                execution_path: std::env::current_dir()
                    .unwrap_or_else(|_| std::path::PathBuf::from("."))
                    .to_string_lossy()
                    .to_string(),
            })
            .collect();

        Ok(history)
    }

    // Configuration methods
    pub async fn get_config(&self, key: &str) -> Result<Option<String>> {
        let result =
            sqlx::query_as::<_, (String,)>("SELECT value FROM configuration WHERE key = ?")
                .bind(key)
                .fetch_optional(&self.pool)
                .await?;

        Ok(result.map(|(value,)| value))
    }

    pub async fn set_config(&self, key: &str, value: &str) -> Result<()> {
        sqlx::query(
            r#"
            INSERT OR REPLACE INTO configuration (key, value)
            VALUES (?, ?)
            "#,
        )
        .bind(key)
        .bind(value)
        .execute(&self.pool)
        .await?;

        Ok(())
    }

    pub async fn get_all_config(&self) -> Result<Vec<(String, String)>> {
        let rows = sqlx::query_as::<_, (String, String)>(
            "SELECT key, value FROM configuration ORDER BY key",
        )
        .fetch_all(&self.pool)
        .await?;

        Ok(rows)
    }
}
