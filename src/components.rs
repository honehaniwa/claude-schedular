use chrono::Local;
use dioxus::prelude::*;

use crate::git::*;
use crate::models::*;
use crate::utils::*;

/// スケジュールを5秒ごとにチェックする関数
fn schedule_checker(
    mut schedules: Signal<Vec<Schedule>>,
    mut execution_history: Signal<Vec<ExecutionHistory>>,
) {
    use_effect(move || {
        spawn(async move {
            loop {
                tokio::time::sleep(tokio::time::Duration::from_secs(5)).await;

                let mut schedules_to_execute = Vec::new();
                schedules.with(|s| {
                    for schedule in s.iter() {
                        if schedule.status == ScheduleStatus::Pending {
                            if let Some(scheduled_time) = &schedule.scheduled_time {
                                if is_time_reached(scheduled_time) {
                                    schedules_to_execute.push(schedule.clone());
                                }
                            }
                        }
                    }
                });

                for schedule in schedules_to_execute {
                    let command = if schedule.is_shell_mode {
                        schedule.command.clone()
                    } else {
                        format!("claude -p \"{}\"", schedule.command)
                    };

                    let result =
                        if schedule.branch != "main" && schedule.branch != get_current_branch() {
                            execute_command_in_worktree(
                                &command,
                                &schedule.branch,
                                schedule.is_shell_mode,
                                &schedule.execution_path,
                            )
                        } else {
                            execute_command_in_directory(&command, &schedule.execution_path)
                        };

                    let (status, output) = match result {
                        Ok(output) => {
                            let stdout = String::from_utf8_lossy(&output.stdout);
                            let stderr = String::from_utf8_lossy(&output.stderr);

                            if output.status.success() {
                                (ScheduleStatus::Completed, format!("{stdout}\n{stderr}"))
                            } else {
                                (ScheduleStatus::Failed, format!("{stdout}\n{stderr}"))
                            }
                        }
                        Err(e) => (ScheduleStatus::Failed, format!("エラー: {e}")),
                    };

                    // スケジュールのステータスを更新
                    schedules.with_mut(|s| {
                        if let Some(sch) = s.iter_mut().find(|sch| sch.id == schedule.id) {
                            sch.status = status.clone();
                        }
                    });

                    // 実行履歴に追加
                    let history = ExecutionHistory {
                        id: format!("history_{}", chrono::Utc::now().timestamp_micros()),
                        command: schedule.command.clone(),
                        executed_at: Local::now().format("%Y-%m-%d %H:%M:%S").to_string(),
                        execution_type: ExecutionType::FromSchedule,
                        status: if status == ScheduleStatus::Completed {
                            ExecutionStatus::Success
                        } else {
                            ExecutionStatus::Failed
                        },
                        output,
                        branch: schedule.branch.clone(),
                        execution_path: schedule.execution_path.clone(),
                    };

                    execution_history.with_mut(|h| h.push(history));
                }
            }
        });
    });
}

/// メインアプリケーション
pub fn app() -> Element {
    let mut text_content = use_signal(String::new);
    // システムのダークモード設定を初期値として設定
    let mut is_dark_mode = use_signal(|| {
        // 初期化時にシステムテーマを同期的に検出
        #[cfg(target_os = "macos")]
        {
            if let Ok(output) = std::process::Command::new("defaults")
                .arg("read")
                .arg("-g")
                .arg("AppleInterfaceStyle")
                .output()
            {
                let theme = String::from_utf8_lossy(&output.stdout);
                return theme.trim() == "Dark";
            }
        }

        #[cfg(target_os = "linux")]
        {
            if let Ok(output) = std::process::Command::new("gsettings")
                .arg("get")
                .arg("org.gnome.desktop.interface")
                .arg("gtk-theme")
                .output()
            {
                let theme = String::from_utf8_lossy(&output.stdout);
                return theme.to_lowercase().contains("dark");
            }
        }

        #[cfg(target_os = "windows")]
        {
            if let Ok(output) = std::process::Command::new("reg")
                .arg("query")
                .arg("HKEY_CURRENT_USER\\SOFTWARE\\Microsoft\\Windows\\CurrentVersion\\Themes\\Personalize")
                .arg("/v")
                .arg("AppsUseLightTheme")
                .output()
            {
                let theme = String::from_utf8_lossy(&output.stdout);
                return theme.contains("0x0");
            }
        }

        false // デフォルトはライトモード
    });
    let mut is_executing = use_signal(|| false);

    let mut schedules = use_signal(Vec::<Schedule>::new);
    let mut use_schedule = use_signal(|| false);
    let mut is_tomorrow = use_signal(|| false);
    let mut selected_hour = use_signal(|| 9u32);
    let mut selected_minute = use_signal(|| 0u32);

    // 実行履歴用の状態
    let mut execution_history = use_signal(Vec::<ExecutionHistory>::new);

    // シェルモード実行用の状態
    let mut use_shell_mode = use_signal(|| false);

    // branch選択用の状態
    let mut available_branches = use_signal(get_git_worktree_branches);
    let mut selected_branch = use_signal(get_current_branch);
    let mut use_git_worktree = use_signal(|| false);
    let mut is_git_repo = use_signal(|| {
        crate::git::is_git_repository(
            std::env::current_dir()
                .unwrap_or_else(|_| std::path::PathBuf::from("."))
                .to_string_lossy()
                .as_ref(),
        )
    });

    // 実行パス用の状態
    let mut execution_path = use_signal(|| {
        std::env::current_dir()
            .unwrap_or_else(|_| std::path::PathBuf::from("."))
            .to_string_lossy()
            .to_string()
    });
    let mut last_execution_path = use_signal(String::new);

    // 定期的なスケジュールチェック（5秒ごと）
    schedule_checker(schedules, execution_history);

    // Claude Code実行関数
    let execute_command = move |_: Event<MouseData>| {
        let prompt = text_content().clone();
        if prompt.trim().is_empty() {
            return;
        }

        is_executing.set(true);

        let shell_mode = use_shell_mode();
        let branch = selected_branch();
        let use_worktree = use_git_worktree();
        let exec_path = execution_path();
        spawn(async move {
            let result = if use_worktree && branch != "main" && branch != get_current_branch() {
                // Git Worktreeを使用
                execute_command_in_worktree(&prompt, &branch, shell_mode, &exec_path)
            } else {
                // 通常の実行
                let command = if shell_mode {
                    prompt.clone()
                } else {
                    format!("claude -p \"{prompt}\"")
                };
                execute_command_in_directory(&command, &exec_path)
            };

            let _ = match result {
                Ok(output) => {
                    let stdout = String::from_utf8_lossy(&output.stdout);
                    let stderr = String::from_utf8_lossy(&output.stderr);

                    let (result_msg, exec_status) = if output.status.success() {
                        (
                            format!(
                                "✅ 実行成功 [{}]:\n{}",
                                Local::now().format("%H:%M:%S"),
                                stdout
                            ),
                            ExecutionStatus::Success,
                        )
                    } else {
                        (
                            format!(
                                "❌ 実行失敗 [{}]:\n{}\n{}",
                                Local::now().format("%H:%M:%S"),
                                stdout,
                                stderr
                            ),
                            ExecutionStatus::Failed,
                        )
                    };

                    // 履歴に記録
                    let history = ExecutionHistory {
                        id: format!("history_{}", chrono::Utc::now().timestamp_micros()),
                        command: prompt.clone(),
                        executed_at: Local::now().format("%Y-%m-%d %H:%M:%S").to_string(),
                        execution_type: if shell_mode {
                            ExecutionType::ShellMode
                        } else {
                            ExecutionType::Manual
                        },
                        status: exec_status,
                        output: format!("{stdout}\n{stderr}"),
                        branch: if use_worktree {
                            branch.clone()
                        } else {
                            get_current_branch()
                        },
                        execution_path: exec_path.clone(),
                    };
                    execution_history.with_mut(|h| h.push(history));

                    result_msg
                }
                Err(e) => {
                    let error_msg =
                        format!("❌ エラー [{}]: {}", Local::now().format("%H:%M:%S"), e);

                    // 履歴に記録
                    let history = ExecutionHistory {
                        id: format!("history_{}", chrono::Utc::now().timestamp_micros()),
                        command: prompt.clone(),
                        executed_at: Local::now().format("%Y-%m-%d %H:%M:%S").to_string(),
                        execution_type: if shell_mode {
                            ExecutionType::ShellMode
                        } else {
                            ExecutionType::Manual
                        },
                        status: ExecutionStatus::Failed,
                        output: format!("エラー: {e}"),
                        branch: if use_worktree {
                            branch.clone()
                        } else {
                            get_current_branch()
                        },
                        execution_path: exec_path,
                    };
                    execution_history.with_mut(|h| h.push(history));

                    error_msg
                }
            };

            is_executing.set(false);
        });
    };

    // スケジュール追加関数
    let add_schedule = move |_: Event<MouseData>| {
        let prompt = text_content().clone();
        if !prompt.trim().is_empty() {
            let scheduled_time = if use_schedule() {
                Some(build_scheduled_time(
                    is_tomorrow(),
                    selected_hour(),
                    selected_minute(),
                ))
            } else {
                None
            };

            let schedule = Schedule {
                id: format!("schedule_{}", chrono::Utc::now().timestamp()),
                command: prompt,
                scheduled_time,
                _memo: String::new(),
                created_at: Local::now().format("%Y-%m-%d %H:%M:%S").to_string(),
                status: ScheduleStatus::Pending,
                is_shell_mode: use_shell_mode(),
                branch: if use_git_worktree() {
                    selected_branch()
                } else {
                    get_current_branch()
                },
                execution_path: execution_path(),
            };

            schedules.with_mut(|s| s.push(schedule));
            text_content.set(String::new());

            // 時間設定をリセット
            use_schedule.set(false);
            is_tomorrow.set(false);
            selected_hour.set(9);
            selected_minute.set(0);
        }
    };

    // branch更新関数
    let refresh_branches = move |_: Event<MouseData>| {
        let path = execution_path();
        let is_repo = crate::git::is_git_repository(&path);
        is_git_repo.set(is_repo);

        if is_repo {
            let branches = crate::git::get_git_worktree_branches_in_directory(&path);
            let current = crate::git::get_current_branch_in_directory(&path);
            available_branches.set(branches);
            selected_branch.set(current);
        } else {
            available_branches.set(vec!["main".to_string()]);
            selected_branch.set("main".to_string());
        }
    };

    // テーマ設定
    let bg_color = if is_dark_mode() { "#1a1a1a" } else { "#ffffff" };
    let text_color = if is_dark_mode() { "#ffffff" } else { "#000000" };
    let card_bg = if is_dark_mode() { "#2a2a2a" } else { "#f0f0f0" };
    let border_color = if is_dark_mode() { "#444444" } else { "#cccccc" };
    let button_bg = if is_dark_mode() { "#333333" } else { "#f0f0f0" };
    let textarea_bg = if is_dark_mode() { "#1a1a1a" } else { "#ffffff" };

    // 基本的なUIコンポーネントを描画
    rsx! {
        head {
            title { "Claude Scheduler" }
            script {
                "
                // システムテーマに基づいて初期テーマを設定
                (function() {{
                    const prefersDark = window.matchMedia('(prefers-color-scheme: dark)').matches;
                    document.documentElement.setAttribute('data-system-theme', prefersDark ? 'dark' : 'light');
                    
                    // システムテーマ変更の監視
                    window.matchMedia('(prefers-color-scheme: dark)').addEventListener('change', function(e) {{
                        document.documentElement.setAttribute('data-system-theme', e.matches ? 'dark' : 'light');
                    }});
                }})();
                "
            }
            style {
                "
                * {{
                    box-sizing: border-box;
                }}
                html, body {{
                    margin: 0;
                    padding: 0;
                    background-color: {bg_color};
                    color: {text_color};
                    font-family: Arial, sans-serif;
                    transition: background-color 0.3s ease, color 0.3s ease;
                }}
                
                /* システムのダークモード設定に基づく初期テーマ */
                @media (prefers-color-scheme: dark) {{
                    :root {{
                        --system-theme: dark;
                    }}
                }}
                
                @media (prefers-color-scheme: light) {{
                    :root {{
                        --system-theme: light;
                    }}
                }}
                "
            }
        }

        div {
            style: "padding: 20px; background-color: {bg_color}; color: {text_color}; min-height: 100vh;",

            h1 {
                style: "margin: 0 0 10px 0; color: {text_color};",
                "🚀 Claude Scheduler"
            }
            p {
                style: "margin: 5px 0; color: {text_color}; opacity: 0.8;",
                "Claude Scheduler: Claude Code自動実行スケジューラー"
            }
            p {
                style: "margin: 5px 0; color: {text_color}; opacity: 0.6; font-size: 0.8rem;",
                "現在時刻: " {get_current_datetime()} " | 登録件数: " {schedules().len().to_string()} " | 5秒間隔で自動チェック"
            }

            button {
                onclick: move |_| is_dark_mode.set(!is_dark_mode()),
                style: "margin: 10px 0; padding: 8px 16px; background: {button_bg}; border: 1px solid {border_color}; border-radius: 4px; cursor: pointer; color: {text_color}; transition: all 0.2s ease;",
                {if is_dark_mode() { "☀️ ライト" } else { "🌙 ダーク" }}
            }

            div {
                style: "background: {card_bg}; padding: 20px; border-radius: 8px; border: 1px solid {border_color}; margin: 20px 0;",

                h3 {
                    style: "margin: 0 0 15px 0; color: {text_color};",
                    "📝 Claude Code 実行・スケジュール登録"
                }

                // 実行パス設定
                div {
                    style: "margin-bottom: 15px; padding: 10px; background: {textarea_bg}; border: 1px solid {border_color}; border-radius: 4px;",

                    div {
                        style: "margin-bottom: 10px;",

                        div {
                            style: "display: flex; align-items: center; gap: 10px; margin-bottom: 8px;",

                            span {
                                style: "font-size: 0.85rem; color: {text_color}; font-weight: bold; min-width: 80px;",
                                "📁 実行パス:"
                            }

                            input {
                                r#type: "text",
                                value: execution_path(),
                                oninput: move |evt| {
                                    let path = crate::utils::expand_path(&evt.value());
                                    execution_path.set(path.clone());

                                    // 有効なパスの場合は最後の実行パスとして保存
                                    if crate::utils::is_valid_directory(&path) {
                                        last_execution_path.set(path.clone());

                                        // gitリポジトリかチェックし、ブランチ情報を更新
                                        let is_repo = crate::git::is_git_repository(&path);
                                        is_git_repo.set(is_repo);

                                        if is_repo {
                                            let branches = crate::git::get_git_worktree_branches_in_directory(&path);
                                            let current = crate::git::get_current_branch_in_directory(&path);
                                            available_branches.set(branches);
                                            selected_branch.set(current);
                                        } else {
                                            available_branches.set(vec!["main".to_string()]);
                                            selected_branch.set("main".to_string());
                                        }
                                    }
                                },
                                placeholder: "実行ディレクトリのパス (例: ~/projects, /path/to/dir)",
                                style: "flex: 1; padding: 6px 8px; border: 1px solid {border_color}; border-radius: 4px; background: {textarea_bg}; color: {text_color}; font-family: monospace; font-size: 0.85rem;",
                            }

                            button {
                                onclick: move |_| {
                                    let current_dir = std::env::current_dir()
                                        .unwrap_or_else(|_| std::path::PathBuf::from("."))
                                        .to_string_lossy()
                                        .to_string();
                                    execution_path.set(current_dir.clone());
                                    last_execution_path.set(current_dir.clone());

                                    // gitリポジトリかチェックし、ブランチ情報を更新
                                    let is_repo = crate::git::is_git_repository(&current_dir);
                                    is_git_repo.set(is_repo);

                                    if is_repo {
                                        let branches = crate::git::get_git_worktree_branches_in_directory(&current_dir);
                                        let current = crate::git::get_current_branch_in_directory(&current_dir);
                                        available_branches.set(branches);
                                        selected_branch.set(current);
                                    } else {
                                        available_branches.set(vec!["main".to_string()]);
                                        selected_branch.set("main".to_string());
                                    }
                                },
                                style: "padding: 4px 8px; background: {button_bg}; border: 1px solid {border_color}; border-radius: 4px; cursor: pointer; color: {text_color}; font-size: 0.8rem;",
                                "📍 現在"
                            }

                            button {
                                onclick: move |_| {
                                                        let mut exec_path = execution_path;
                    let mut last_exec_path = last_execution_path;
                    let mut repo_state = is_git_repo;
                    let mut branches = available_branches;
                    let mut current_branch = selected_branch;

                                    spawn(async move {
                                        if let Some(folder) = rfd::AsyncFileDialog::new()
                                            .set_title("実行ディレクトリを選択")
                                            .pick_folder()
                                            .await
                                        {
                                            let path = folder.path().to_string_lossy().to_string();
                                            exec_path.set(path.clone());
                                            last_exec_path.set(path.clone());

                                            // gitリポジトリかチェックし、ブランチ情報を更新
                                            let is_repo = crate::git::is_git_repository(&path);
                                            repo_state.set(is_repo);

                                            if is_repo {
                                                let repo_branches = crate::git::get_git_worktree_branches_in_directory(&path);
                                                let current = crate::git::get_current_branch_in_directory(&path);
                                                branches.set(repo_branches);
                                                current_branch.set(current);
                                            } else {
                                                branches.set(vec!["main".to_string()]);
                                                current_branch.set("main".to_string());
                                            }
                                        }
                                    });
                                },
                                style: "padding: 4px 8px; background: {button_bg}; border: 1px solid {border_color}; border-radius: 4px; cursor: pointer; color: {text_color}; font-size: 0.8rem;",
                                "📂 選択"
                            }

                            if !last_execution_path().is_empty() && last_execution_path() != execution_path() {
                                button {
                                    onclick: move |_| execution_path.set(last_execution_path()),
                                    style: "padding: 4px 8px; background: {button_bg}; border: 1px solid {border_color}; border-radius: 4px; cursor: pointer; color: {text_color}; font-size: 0.8rem;",
                                    "🔄 前回"
                                }
                            }
                        }

                        div {
                            style: "font-size: 0.7rem; opacity: 0.8; color: {text_color}; margin-left: 88px;",
                            if crate::utils::is_valid_directory(&execution_path()) {
                                if is_git_repo() {
                                    span {
                                        style: "color: #16a34a;",
                                        "✅ 有効なディレクトリ (Gitリポジトリ)"
                                    }
                                } else {
                                    span {
                                        style: "color: #f59e0b;",
                                        "✅ 有効なディレクトリ (Gitリポジトリではありません)"
                                    }
                                }
                            } else {
                                span {
                                    style: "color: #dc2626;",
                                    "❌ ディレクトリが見つかりません"
                                }
                            }
                        }

                        p {
                            style: "margin: 6px 0 0 88px; font-size: 0.7rem; color: {text_color}; opacity: 0.6;",
                            "💡 claude コマンドを実行するディレクトリを指定します。~/は自動展開されます。"
                        }
                    }
                }

                // Git Worktree機能切り替え
                div {
                    style: "margin-bottom: 15px; padding: 10px; background: {textarea_bg}; border: 1px solid {border_color}; border-radius: 4px;",

                    div {
                        style: "display: flex; justify-content: space-between; align-items: center;",

                        label {
                            style: format!("display: flex; align-items: center; cursor: {}; color: {}; font-size: 0.85rem;",
                                if is_git_repo() { "pointer" } else { "not-allowed" },
                                if is_git_repo() { text_color } else { "#888888" }),
                            input {
                                r#type: "checkbox",
                                checked: use_git_worktree() && is_git_repo(),
                                disabled: !is_git_repo(),
                                onchange: move |evt| {
                                    if is_git_repo() {
                                        use_git_worktree.set(evt.checked());
                                    }
                                },
                                style: "margin-right: 8px; transform: scale(0.9);",
                            }
                            span {
                                style: format!("font-weight: bold; color: {};",
                                    if is_git_repo() { text_color } else { "#888888" }),
                                if is_git_repo() {
                                    "🌿 Git Worktree並列実行"
                                } else {
                                    "🌿 Git Worktree並列実行 (Gitリポジトリが必要)"
                                }
                            }
                        }

                        if use_git_worktree() && is_git_repo() {
                            button {
                                onclick: refresh_branches,
                                style: "padding: 4px 8px; background: {button_bg}; border: 1px solid {border_color}; border-radius: 4px; cursor: pointer; color: {text_color}; font-size: 0.8rem;",
                                "🔄 更新"
                            }
                        }
                    }

                    if use_git_worktree() && is_git_repo() {
                        div {
                            style: "margin-top: 10px;",

                            div {
                                style: "display: flex; align-items: center; gap: 10px;",

                                span {
                                    style: "font-size: 0.8rem; color: {text_color}; opacity: 0.8; min-width: 60px;",
                                    "Branch:"
                                }

                                select {
                                    value: selected_branch(),
                                    onchange: move |evt| selected_branch.set(evt.value()),
                                    style: "flex: 1; padding: 6px; border: 1px solid {border_color}; border-radius: 4px; background: {textarea_bg}; color: {text_color}; font-family: monospace; font-size: 0.85rem;",

                                    for branch in available_branches().iter() {
                                        option {
                                            value: "{branch}",
                                            {branch.clone()}
                                        }
                                    }
                                }
                            }

                            p {
                                style: "margin: 6px 0 0 0; font-size: 0.7rem; color: {text_color}; opacity: 0.6;",
                                "💡 選択したbranchのworktreeで並列実行されます（指定ディレクトリ基準）"
                            }
                        }
                    }
                }

                div {
                    style: "position: relative;",

                    // モード表示とチェックボックス
                    div {
                        style: "display: flex; justify-content: space-between; align-items: center; margin-bottom: 8px;",

                        // 現在のモード表示
                        div {
                            style: "display: flex; align-items: center; font-size: 0.9rem; font-weight: bold; color: {text_color};",
                            if use_shell_mode() {
                                span {
                                    style: "color: #16a34a; margin-right: 8px;",
                                    "💻 Shell Mode"
                                }
                            } else {
                                span {
                                    style: "color: #3b82f6; margin-right: 8px;",
                                    "🤖 Claude Code Mode"
                                }
                            }

                            span {
                                style: "font-size: 0.75rem; opacity: 0.7; color: {text_color};",
                                if use_shell_mode() {
                                    "直接シェルコマンド実行"
                                } else {
                                    "Claude AIに送信"
                                }
                            }
                        }

                        // コンパクトなモード切り替え
                        label {
                            style: "display: flex; align-items: center; cursor: pointer; color: {text_color}; font-size: 0.85rem;",
                            input {
                                r#type: "checkbox",
                                checked: use_shell_mode(),
                                onchange: move |evt| use_shell_mode.set(evt.checked()),
                                style: "margin-right: 6px; transform: scale(0.9);",
                            }
                            span {
                                style: "color: {text_color}; opacity: 0.8;",
                                "Shell Mode"
                            }
                        }
                    }

                    textarea {
                        value: text_content(),
                        oninput: move |evt| text_content.set(evt.value()),
                        placeholder: if use_shell_mode() { "シェルコマンドを入力（例: ls -la, echo 'hello'）..." } else { "Claude Codeプロンプトを入力..." },
                        style: "width: 100%; height: 100px; padding: 10px; border: 1px solid {border_color}; border-radius: 4px; font-family: monospace; background: {textarea_bg}; color: {text_color}; resize: vertical; transition: all 0.2s ease;",
                    }
                }

                // スケジュール設定
                div {
                    style: "margin: 15px 0; padding: 15px; background: {textarea_bg}; border: 1px solid {border_color}; border-radius: 4px;",

                    label {
                        style: "display: flex; align-items: center; margin-bottom: 15px; cursor: pointer; color: {text_color};",
                        input {
                            r#type: "checkbox",
                            checked: use_schedule(),
                            onchange: move |evt| use_schedule.set(evt.checked()),
                            style: "margin-right: 8px;",
                        }
                        span {
                            style: "font-weight: bold; color: {text_color};",
                            "⏰ 時間指定自動実行を有効にする"
                        }
                    }

                    if use_schedule() {
                        div {
                            style: "margin-left: 20px;",

                            div {
                                style: "margin-bottom: 15px;",
                                h4 {
                                    style: "margin: 0 0 8px 0; color: {text_color}; font-size: 0.9rem;",
                                    "📅 実行日"
                                }

                                label {
                                    style: "margin-right: 15px; cursor: pointer; color: {text_color};",
                                    input {
                                        r#type: "radio",
                                        name: "date_choice",
                                        checked: !is_tomorrow(),
                                        onchange: move |_| is_tomorrow.set(false),
                                        style: "margin-right: 5px;",
                                    }
                                    "今日"
                                }

                                label {
                                    style: "cursor: pointer; color: {text_color};",
                                    input {
                                        r#type: "radio",
                                        name: "date_choice",
                                        checked: is_tomorrow(),
                                        onchange: move |_| is_tomorrow.set(true),
                                        style: "margin-right: 5px;",
                                    }
                                    "明日"
                                }
                            }

                            div {
                                style: "margin-bottom: 15px;",
                                h4 {
                                    style: "margin: 0 0 8px 0; color: {text_color}; font-size: 0.9rem;",
                                    "🕐 実行時刻"
                                }

                                div {
                                    style: "display: flex; gap: 10px; align-items: center; margin-bottom: 10px;",

                                    select {
                                        value: selected_hour().to_string(),
                                        onchange: move |evt| {
                                            if let Ok(hour) = evt.value().parse::<u32>() {
                                                selected_hour.set(hour);
                                            }
                                        },
                                        style: "padding: 8px; border: 1px solid {border_color}; border-radius: 4px; background: {textarea_bg}; color: {text_color}; font-family: monospace;",

                                        // 一部の時間オプション（省略版）
                                        option { value: "9", "09時" }
                                        option { value: "12", "12時" }
                                        option { value: "15", "15時" }
                                        option { value: "18", "18時" }
                                        option { value: "21", "21時" }
                                    }

                                    span {
                                        style: "color: {text_color}; font-weight: bold;",
                                        ":"
                                    }

                                    select {
                                        value: selected_minute().to_string(),
                                        onchange: move |evt| {
                                            if let Ok(minute) = evt.value().parse::<u32>() {
                                                selected_minute.set(minute);
                                            }
                                        },
                                        style: "padding: 8px; border: 1px solid {border_color}; border-radius: 4px; background: {textarea_bg}; color: {text_color}; font-family: monospace;",

                                        // 一部の分オプション（省略版）
                                        option { value: "0", "00分" }
                                        option { value: "15", "15分" }
                                        option { value: "30", "30分" }
                                        option { value: "45", "45分" }
                                    }
                                }

                                p {
                                    style: "margin: 10px 0 0 0; padding: 8px; background: {card_bg}; border: 1px solid {border_color}; border-radius: 4px; font-size: 0.9rem; color: {text_color}; font-weight: bold;",
                                    "⏰ 予定時刻: "
                                    {if is_tomorrow() { "明日" } else { "今日" }}
                                    " "
                                    {format!("{:02}:{:02}", selected_hour(), selected_minute())}
                                }
                            }
                        }
                    }
                }

                div {
                    style: "display: flex; justify-content: space-between; align-items: center; margin: 15px 0;",

                    span {
                        style: "font-size: 0.9rem; color: {text_color}; opacity: 0.7;",
                        "文字数: " {text_content().len().to_string()}
                    }

                    div {
                        style: "display: flex; gap: 10px;",

                        button {
                            onclick: execute_command,
                            disabled: is_executing() || text_content().trim().is_empty(),
                            style: "padding: 8px 16px; background: #16a34a; color: white; border: none; border-radius: 4px; cursor: pointer; font-weight: 500; transition: all 0.2s ease;",
                            {if is_executing() { "実行中..." } else { "▶️ 即座実行" }}
                        }

                        button {
                            onclick: add_schedule,
                            disabled: text_content().trim().is_empty(),
                            style: "padding: 8px 16px; background: #3b82f6; color: white; border: none; border-radius: 4px; cursor: pointer; font-weight: 500; transition: all 0.2s ease;",
                            {if use_schedule() { "📅 スケジュール登録" } else { "📋 リスト追加" }}
                        }

                        button {
                            onclick: move |_: Event<MouseData>| text_content.set(String::new()),
                            style: "padding: 8px 16px; background: #dc2626; color: white; border: none; border-radius: 4px; cursor: pointer; font-weight: 500; transition: all 0.2s ease;",
                            "🗑️ クリア"
                        }
                    }
                }
            }

            // スケジュール一覧
            div {
                style: "background: {card_bg}; padding: 20px; border-radius: 8px; border: 1px solid {border_color}; margin: 20px 0;",

                h3 {
                    style: "margin: 0 0 15px 0; color: {text_color};",
                    "📋 登録済みスケジュール (" {schedules().len().to_string()} "件)"
                }

                if schedules().is_empty() {
                    div {
                        style: "text-align: center; padding: 40px; opacity: 0.6; color: {text_color};",
                        "📝 スケジュールが登録されていません"
                    }
                } else {
                    for schedule in schedules().iter() {
                        div {
                            key: "{schedule.id}",
                            style: "background: {textarea_bg}; padding: 15px; border-radius: 4px; border: 1px solid {border_color}; margin-bottom: 10px;",

                            div {
                                style: "font-weight: bold; color: {text_color}; margin-bottom: 8px; font-size: 0.95rem; word-break: break-word;",
                                "💬 " {schedule.command.clone()}
                            }

                                                                    div {
                                            style: "font-size: 0.8rem; opacity: 0.8; color: {text_color}; margin-bottom: 5px;",
                                            "📅 作成: " {schedule.created_at.clone()} " | 状態: " {schedule.status.to_string()} " | "
                                            span {
                                                style: if schedule.is_shell_mode { "color: #16a34a; font-weight: bold;" } else { "color: #3b82f6; font-weight: bold;" },
                                                {if schedule.is_shell_mode { "💻 Shell" } else { "🤖 Claude" }}
                                            }
                                            " | 🌿 Branch: "
                                            span {
                                                style: "color: #8b5cf6; font-weight: bold;",
                                                {schedule.branch.clone()}
                                            }
                                        }

                                        div {
                                            style: "font-size: 0.75rem; opacity: 0.7; color: {text_color}; margin-bottom: 5px;",
                                            "📁 実行パス: "
                                            span {
                                                style: "color: #f59e0b; font-weight: bold; font-family: monospace;",
                                                {schedule.execution_path.clone()}
                                            }
                                        }

                            if let Some(scheduled_time) = &schedule.scheduled_time {
                                div {
                                    style: "font-size: 0.8rem; color: #3b82f6; margin: 5px 0; font-weight: 500;",
                                    "⏰ 実行予定: " {scheduled_time.clone()}
                                }
                            } else {
                                div {
                                    style: "font-size: 0.8rem; opacity: 0.6; color: {text_color}; margin: 5px 0;",
                                    "⚡ 手動実行のみ"
                                }
                            }

                            div {
                                style: "display: flex; gap: 10px; margin-top: 10px;",

                                button {
                                    onclick: {
                                        let schedule_id = schedule.id.clone();
                                        move |_: Event<MouseData>| {
                                            schedules.with_mut(|s| {
                                                s.retain(|sched| sched.id != schedule_id);
                                            });
                                        }
                                    },
                                    style: "padding: 6px 12px; background: #dc2626; color: white; border: none; border-radius: 4px; cursor: pointer; font-size: 0.85rem; font-weight: 500;",
                                    "🗑️ 削除"
                                }
                            }
                        }
                    }
                }
            }

            // 実行履歴
            div {
                style: "background: {card_bg}; padding: 20px; border-radius: 8px; border: 1px solid {border_color}; margin: 20px 0;",

                div {
                    style: "display: flex; justify-content: space-between; align-items: center; margin-bottom: 15px;",
                    h3 {
                        style: "margin: 0; color: {text_color};",
                        "📊 実行履歴・結果 (" {execution_history().len().to_string()} "件)"
                    }

                    if !execution_history().is_empty() {
                        button {
                            onclick: move |_: Event<MouseData>| execution_history.set(Vec::new()),
                            style: "padding: 6px 12px; background: #dc2626; color: white; border: none; border-radius: 4px; cursor: pointer; font-size: 0.85rem; font-weight: 500;",
                            "🗑️ 履歴をクリア"
                        }
                    }
                }

                if execution_history().is_empty() && !is_executing() {
                    div {
                        style: "text-align: center; padding: 40px; opacity: 0.6; color: {text_color};",
                        "📈 実行履歴がありません"
                    }
                } else {
                    div {
                        style: "max-height: 400px; overflow-y: auto;",

                        // 実行中のメッセージを表示
                        if is_executing() {
                            div {
                                style: "background: {textarea_bg}; padding: 15px; border-radius: 4px; border: 1px solid {border_color}; margin-bottom: 10px; border-left: 4px solid #f59e0b;",

                                div {
                                    style: "display: flex; align-items: center; margin-bottom: 8px;",

                                    div {
                                        style: "flex: 1;",
                                        div {
                                            style: "font-weight: bold; color: {text_color}; margin-bottom: 4px; font-size: 0.95rem;",
                                            "💬 " {text_content()}
                                        }

                                        div {
                                            style: "font-size: 0.8rem; opacity: 0.8; color: {text_color}; margin-bottom: 5px;",
                                            "📅 実行中: " {Local::now().format("%Y-%m-%d %H:%M:%S").to_string()} " | "
                                            span {
                                                style: if use_shell_mode() { "color: #16a34a; font-weight: bold;" } else { "color: #3b82f6; font-weight: bold;" },
                                                {if use_shell_mode() { "💻 Shell" } else { "🤖 Claude" }}
                                            }
                                            " | 🌿 Branch: "
                                            span {
                                                style: "color: #8b5cf6; font-weight: bold;",
                                                {if use_git_worktree() { selected_branch() } else { get_current_branch() }}
                                            }
                                            " | 状態: "
                                            span {
                                                style: "color: #f59e0b; font-weight: bold;",
                                                "実行中..."
                                            }
                                        }

                                        div {
                                            style: "font-size: 0.75rem; opacity: 0.7; color: {text_color}; margin-bottom: 5px;",
                                            "📁 実行パス: "
                                            span {
                                                style: "color: #f59e0b; font-weight: bold; font-family: monospace;",
                                                {execution_path()}
                                            }
                                        }
                                    }

                                    div {
                                        style: "margin-left: 10px;",
                                        "⏳"
                                    }
                                }

                                div {
                                    style: "font-size: 0.85rem; color: {text_color}; opacity: 0.8; margin-bottom: 5px;",
                                    "📋 実行結果:"
                                }

                                div {
                                    style: "background: {card_bg}; padding: 10px; border-radius: 4px; border: 1px solid {border_color}; font-family: monospace; color: {text_color}; font-size: 0.75rem;",
                                    "実行中です..."
                                }
                            }
                        }

                        for (index, history) in execution_history().iter().rev().take(10).enumerate() {
                            div {
                                key: "{history.id}",
                                style: format!("background: {}; padding: 15px; border-radius: 4px; border: 1px solid {}; margin-bottom: 10px;{}",
                                    textarea_bg, border_color, if index == 0 { " border-left: 4px solid #16a34a;" } else { "" }),

                                div {
                                    style: "display: flex; justify-content: space-between; align-items: start; margin-bottom: 8px;",

                                    div {
                                        style: "flex: 1;",
                                        div {
                                            style: "font-weight: bold; color: {text_color}; margin-bottom: 4px; font-size: 0.95rem; word-break: break-word;",
                                            "💬 " {history.command.clone()}
                                        }

                                        div {
                                            style: "font-size: 0.8rem; opacity: 0.8; color: {text_color}; margin-bottom: 5px;",
                                            "📅 実行: " {history.executed_at.clone()} " | "
                                            span {
                                                style: match history.execution_type {
                                                    ExecutionType::ShellMode => "color: #16a34a; font-weight: bold;",
                                                    ExecutionType::Manual => "color: #3b82f6; font-weight: bold;",
                                                    ExecutionType::Auto => "color: #3b82f6; font-weight: bold;",
                                                    ExecutionType::FromSchedule => "color: #3b82f6; font-weight: bold;",
                                                },
                                                {match history.execution_type {
                                                    ExecutionType::ShellMode => "💻 Shell",
                                                    ExecutionType::Manual => "🤖 Claude",
                                                    ExecutionType::Auto => "🤖 Claude自動",
                                                    ExecutionType::FromSchedule => "🤖 Claude予約",
                                                }}
                                            }
                                            " | 🌿 Branch: "
                                            span {
                                                style: "color: #8b5cf6; font-weight: bold;",
                                                {history.branch.clone()}
                                            }
                                            " | 結果: "
                                            span {
                                                style: if history.status == ExecutionStatus::Success { "color: #16a34a; font-weight: bold;" } else { "color: #dc2626; font-weight: bold;" },
                                                {history.status.to_string()}
                                            }
                                        }

                                        div {
                                            style: "font-size: 0.75rem; opacity: 0.7; color: {text_color}; margin-bottom: 5px;",
                                            "📁 実行パス: "
                                            span {
                                                style: "color: #f59e0b; font-weight: bold; font-family: monospace;",
                                                {history.execution_path.clone()}
                                            }
                                        }
                                    }

                                    button {
                                        onclick: {
                                            let cmd = history.command.clone();
                                            let path = history.execution_path.clone();
                                            move |_: Event<MouseData>| {
                                                text_content.set(cmd.clone());
                                                execution_path.set(path.clone());
                                                // 再利用時は前回実行パスとして保存
                                                last_execution_path.set(path.clone());
                                            }
                                        },
                                        style: "padding: 4px 8px; background: #3b82f6; color: white; border: none; border-radius: 4px; cursor: pointer; font-size: 0.8rem; font-weight: 500; margin-left: 10px;",
                                        "🔄 再利用"
                                    }
                                }

                                if !history.output.trim().is_empty() {
                                    div {
                                        style: "margin-top: 8px;",

                                        div {
                                            style: "font-size: 0.85rem; color: {text_color}; opacity: 0.8; margin-bottom: 5px;",
                                            "📋 実行結果:"
                                        }

                                        pre {
                                            style: format!("background: {}; padding: 10px; border-radius: 4px; border: 1px solid {}; font-family: monospace; white-space: pre-wrap; color: {}; font-size: 0.75rem; max-height: 200px; overflow-y: auto; margin-top: 5px;{}",
                                                card_bg, border_color, text_color, if history.status == ExecutionStatus::Success { " border-left: 3px solid #16a34a;" } else { " border-left: 3px solid #dc2626;" }),
                                            {history.output.clone()}
                                        }
                                    }
                                }
                            }
                        }

                        if execution_history().len() > 10 {
                            div {
                                style: "text-align: center; padding: 10px; opacity: 0.6; color: {text_color}; font-size: 0.85rem;",
                                "最新10件を表示中（全" {execution_history().len().to_string()} "件）"
                            }
                        }
                    }
                }
            }
        }
    }
}
