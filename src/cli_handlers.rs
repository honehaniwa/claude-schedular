use anyhow::Result;
use chrono::{Local, NaiveDate, NaiveDateTime};
use comfy_table::{ContentArrangement, Table};

use crate::database::Database;
use crate::git;
use crate::models::{ExecutionHistory, ExecutionStatus, ExecutionType, ScheduleStatus};

pub async fn list_schedules(
    db: &Database,
    status_filter: Option<&str>,
    format: &str,
    limit: Option<usize>,
) -> Result<()> {
    let status = status_filter.map(ScheduleStatus::from_string);
    let schedules = db.get_schedules(status, limit).await?;

    match format {
        "json" => {
            let json = serde_json::to_string_pretty(&schedules)?;
            println!("{json}");
        }
        "csv" => {
            let mut wtr = csv::Writer::from_writer(std::io::stdout());
            wtr.write_record([
                "ID",
                "Command",
                "Scheduled Time",
                "Status",
                "Mode",
                "Branch",
                "Created At",
            ])?;

            for schedule in schedules {
                wtr.write_record([
                    &schedule.id,
                    &schedule.command,
                    &schedule.scheduled_time.unwrap_or_default(),
                    &schedule.status.to_db_string(),
                    &if schedule.is_shell_mode {
                        "shell".to_string()
                    } else {
                        "claude".to_string()
                    },
                    &schedule.branch,
                    &schedule.created_at,
                ])?;
            }

            wtr.flush()?;
        }
        _ => {
            // Table format
            let mut table = Table::new();
            table.set_content_arrangement(ContentArrangement::Dynamic);
            table.set_header(vec![
                "ID",
                "Command",
                "Scheduled Time",
                "Status",
                "Mode",
                "Branch",
            ]);

            for schedule in schedules {
                let id_short = if schedule.id.len() > 20 {
                    format!("{}...", &schedule.id[..20])
                } else {
                    schedule.id.clone()
                };

                let command_short = if schedule.command.len() > 30 {
                    format!("{}...", &schedule.command[..30])
                } else {
                    schedule.command.clone()
                };

                table.add_row(vec![
                    id_short,
                    command_short,
                    schedule.scheduled_time.unwrap_or_default(),
                    schedule.status.to_db_string(),
                    if schedule.is_shell_mode {
                        "shell".to_string()
                    } else {
                        "claude".to_string()
                    },
                    schedule.branch,
                ]);
            }

            println!("{table}");
        }
    }

    Ok(())
}

#[allow(clippy::too_many_arguments)]
pub async fn show_history(
    db: &Database,
    status_filter: Option<&str>,
    type_filter: Option<&str>,
    branch_filter: Option<&str>,
    format: &str,
    limit: Option<usize>,
    from_date: Option<NaiveDate>,
    to_date: Option<NaiveDate>,
) -> Result<()> {
    let status = status_filter.map(ExecutionStatus::from_string);
    let exec_type = type_filter.map(ExecutionType::from_string);

    let from = from_date.map(|d| {
        d.and_hms_opt(0, 0, 0)
            .unwrap()
            .and_local_timezone(Local)
            .unwrap()
    });
    let to = to_date.map(|d| {
        d.and_hms_opt(23, 59, 59)
            .unwrap()
            .and_local_timezone(Local)
            .unwrap()
    });

    let history = db
        .get_execution_history(
            status,
            exec_type,
            branch_filter.map(|s| s.to_string()),
            from,
            to,
            limit,
        )
        .await?;

    match format {
        "json" => {
            let json = serde_json::to_string_pretty(&history)?;
            println!("{json}");
        }
        "csv" => {
            let mut wtr = csv::Writer::from_writer(std::io::stdout());
            wtr.write_record([
                "ID",
                "Command",
                "Executed At",
                "Type",
                "Status",
                "Branch",
                "Output",
            ])?;

            for entry in history {
                wtr.write_record([
                    &entry.id,
                    &entry.command,
                    &entry.executed_at,
                    &entry.execution_type.to_db_string(),
                    &entry.status.to_db_string(),
                    &entry.branch,
                    &entry.output,
                ])?;
            }

            wtr.flush()?;
        }
        _ => {
            // Table format
            let mut table = Table::new();
            table.set_content_arrangement(ContentArrangement::Dynamic);
            table.set_header(vec!["Time", "Command", "Type", "Status", "Branch"]);

            for entry in history {
                let command_short = if entry.command.len() > 40 {
                    format!("{}...", &entry.command[..40])
                } else {
                    entry.command.clone()
                };

                let status_emoji = match entry.status {
                    ExecutionStatus::Success => "✅",
                    ExecutionStatus::Failed => "❌",
                };

                table.add_row(vec![
                    entry.executed_at,
                    command_short,
                    entry.execution_type.to_db_string(),
                    format!("{} {}", status_emoji, entry.status),
                    entry.branch,
                ]);
            }

            println!("{table}");
        }
    }

    Ok(())
}

pub async fn run_daemon(
    db: &Database,
    port: u16,
    interval: u64,
    pid_file: Option<&str>,
    log_file: Option<&str>,
    _detach: bool,
) -> Result<()> {
    use std::fs::File;
    use std::io::Write;
    use tokio::time::{interval as tokio_interval, Duration};

    // Write PID file if requested
    if let Some(pid_path) = pid_file {
        let pid = std::process::id();
        let mut file = File::create(pid_path)?;
        writeln!(file, "{pid}")?;
    }

    // Setup logging
    if let Some(_log_path) = log_file {
        // TODO: Implement file logging
        eprintln!("File logging not yet implemented. Using stdout/stderr.");
    }

    println!("🚀 Starting Claude Scheduler daemon...");
    println!("  Port: {port}");
    println!("  Check interval: {interval}s");

    // Schedule checker loop
    let mut interval_timer = tokio_interval(Duration::from_secs(interval));

    loop {
        interval_timer.tick().await;

        // Check for pending schedules
        let schedules = db
            .get_schedules(Some(ScheduleStatus::Pending), None)
            .await?;

        let now = Local::now();

        for schedule in schedules {
            if let Some(scheduled_time_str) = &schedule.scheduled_time {
                if let Ok(scheduled_time) =
                    NaiveDateTime::parse_from_str(scheduled_time_str, "%Y-%m-%dT%H:%M")
                {
                    let scheduled_local = scheduled_time.and_local_timezone(Local).unwrap();

                    if now >= scheduled_local {
                        println!("⏰ Executing scheduled command: {}", schedule.command);

                        // Execute the command
                        let execution_path = if git::is_git_repository(&schedule.execution_path) {
                            git::get_worktree_path(&schedule.branch)
                                .unwrap_or(schedule.execution_path.clone())
                        } else {
                            schedule.execution_path.clone()
                        };

                        let (success, output) = crate::cli_commands::execute_command_internal(
                            &schedule.command,
                            schedule.is_shell_mode,
                            &execution_path,
                            schedule.claude_skip_permissions,
                            schedule.claude_continue_from_last,
                        )
                        .await?;

                        // Update schedule status
                        let new_status = if success {
                            ScheduleStatus::Completed
                        } else {
                            ScheduleStatus::Failed
                        };

                        db.update_schedule_status(&schedule.id, new_status).await?;

                        // Create execution history
                        let history = ExecutionHistory {
                            id: format!(
                                "exec_{}",
                                chrono::Utc::now().timestamp_nanos_opt().unwrap_or(0)
                            ),
                            command: schedule.command.clone(),
                            executed_at: Local::now().format("%Y-%m-%d %H:%M:%S").to_string(),
                            execution_type: ExecutionType::FromSchedule,
                            status: if success {
                                ExecutionStatus::Success
                            } else {
                                ExecutionStatus::Failed
                            },
                            output,
                            branch: schedule.branch.clone(),
                            execution_path,
                            claude_skip_permissions: schedule.claude_skip_permissions,
                            claude_continue_from_last: schedule.claude_continue_from_last,
                        };

                        db.create_execution_history(&history).await?;

                        println!(
                            "  Status: {}",
                            if success { "✅ Success" } else { "❌ Failed" }
                        );
                    }
                }
            }
        }
    }
}
