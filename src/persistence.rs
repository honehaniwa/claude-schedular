use crate::models::{ExecutionHistory, Schedule};
use anyhow::Result;
use std::fs;
use std::path::PathBuf;

/// 実行履歴とスケジュールのJSONファイル保存先ディレクトリを取得
fn get_data_dir() -> Result<PathBuf> {
    let config_dir = crate::config::get_config_dir()?;
    let data_dir = config_dir.join("data");
    if !data_dir.exists() {
        fs::create_dir_all(&data_dir)?;
    }
    Ok(data_dir)
}

/// 実行履歴をJSONファイルに保存
pub fn save_execution_history(history: &[ExecutionHistory]) -> Result<()> {
    let data_dir = get_data_dir()?;
    let history_file = data_dir.join("execution_history.json");

    let json = serde_json::to_string_pretty(history)?;
    fs::write(history_file, json)?;

    Ok(())
}

/// 実行履歴をJSONファイルから読み込み
pub fn load_execution_history() -> Result<Vec<ExecutionHistory>> {
    let data_dir = get_data_dir()?;
    let history_file = data_dir.join("execution_history.json");

    if !history_file.exists() {
        return Ok(Vec::new());
    }

    let json = fs::read_to_string(history_file)?;
    let history: Vec<ExecutionHistory> = serde_json::from_str(&json)?;

    Ok(history)
}

/// スケジュールをJSONファイルに保存
pub fn save_schedules(schedules: &[Schedule]) -> Result<()> {
    let data_dir = get_data_dir()?;
    let schedules_file = data_dir.join("schedules.json");

    let json = serde_json::to_string_pretty(schedules)?;
    fs::write(schedules_file, json)?;

    Ok(())
}

/// スケジュールをJSONファイルから読み込み
pub fn load_schedules() -> Result<Vec<Schedule>> {
    let data_dir = get_data_dir()?;
    let schedules_file = data_dir.join("schedules.json");

    if !schedules_file.exists() {
        return Ok(Vec::new());
    }

    let json = fs::read_to_string(schedules_file)?;
    let schedules: Vec<Schedule> = serde_json::from_str(&json)?;

    Ok(schedules)
}

/// データディレクトリのクリーンアップ（古いデータの削除）
pub fn cleanup_old_data(days: u64) -> Result<()> {
    use chrono::{Duration, Local};

    let cutoff_date = Local::now() - Duration::days(days as i64);
    let cutoff_str = cutoff_date.format("%Y-%m-%d %H:%M:%S").to_string();

    // 実行履歴をクリーンアップ
    let mut history = load_execution_history()?;
    history.retain(|h| h.executed_at > cutoff_str);
    save_execution_history(&history)?;

    Ok(())
}
