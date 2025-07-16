use chrono::{Duration, Local, NaiveDateTime, TimeZone};
use std::process::{Command, Output};

/// 現在時刻を"YYYY-MM-DDTHH:MM"形式で取得
#[allow(dead_code)]
pub fn get_current_datetime() -> String {
    Local::now().format("%Y-%m-%dT%H:%M").to_string()
}

/// 時刻文字列を比較
#[allow(dead_code)]
pub fn is_time_reached(scheduled_time: &str) -> bool {
    if let Ok(scheduled_dt) = NaiveDateTime::parse_from_str(scheduled_time, "%Y-%m-%dT%H:%M") {
        if let Some(scheduled_local) = Local.from_local_datetime(&scheduled_dt).single() {
            return Local::now() >= scheduled_local;
        }
    }
    false
}

/// 今日/明日 + 時刻から datetime-local形式に変換
#[allow(dead_code)]
pub fn build_scheduled_time(is_tomorrow: bool, hour: u32, minute: u32) -> String {
    let now = Local::now();
    let target_date = if is_tomorrow {
        now + Duration::days(1)
    } else {
        now
    };

    target_date
        .date_naive()
        .and_hms_opt(hour, minute, 0)
        .unwrap_or(target_date.naive_local())
        .format("%Y-%m-%dT%H:%M")
        .to_string()
}

/// 指定されたディレクトリでコマンドを実行
#[allow(dead_code)]
pub fn execute_command_in_directory(
    command: &str,
    directory: &str,
) -> Result<Output, std::io::Error> {
    let mut cmd = Command::new("sh");
    cmd.arg("-c").arg(command);

    // 指定されたディレクトリが存在するかチェック
    if std::path::Path::new(directory).exists() {
        cmd.current_dir(directory);
    }

    cmd.output()
}

/// パスが有効なディレクトリかどうかをチェック
#[allow(dead_code)]
pub fn is_valid_directory(path: &str) -> bool {
    std::path::Path::new(path).is_dir()
}

/// ホームディレクトリからの相対パス展開
pub fn expand_path(path: &str) -> String {
    if path.starts_with("~/") {
        if let Some(home) = std::env::var_os("HOME") {
            return format!("{}{}", home.to_string_lossy(), &path[1..]);
        }
    }
    path.to_string()
}
