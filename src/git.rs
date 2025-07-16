use std::process::Command;

/// 指定されたディレクトリがgitリポジトリかチェック
pub fn is_git_repository(directory: &str) -> bool {
    let expanded_path = crate::utils::expand_path(directory);

    if !std::path::Path::new(&expanded_path).exists() {
        return false;
    }

    match Command::new("git")
        .current_dir(&expanded_path)
        .arg("rev-parse")
        .arg("--git-dir")
        .output()
    {
        Ok(output) => output.status.success(),
        Err(_) => false,
    }
}

/// 指定されたディレクトリのgit worktreeのbranchを取得する関数
pub fn get_git_worktree_branches_in_directory(directory: &str) -> Vec<String> {
    let expanded_path = crate::utils::expand_path(directory);

    if !is_git_repository(&expanded_path) {
        return vec!["main".to_string()];
    }

    match Command::new("git")
        .current_dir(&expanded_path)
        .arg("worktree")
        .arg("list")
        .arg("--porcelain")
        .output()
    {
        Ok(output) => {
            let stdout = String::from_utf8_lossy(&output.stdout);
            let mut branches = Vec::new();

            for line in stdout.lines() {
                if line.starts_with("branch ") {
                    let branch_name = line.strip_prefix("branch ").unwrap_or("main");
                    if !branch_name.is_empty() {
                        branches.push(branch_name.to_string());
                    }
                }
            }

            if branches.is_empty() {
                // フォールバック: 通常のbranchを取得
                if let Ok(branch_output) = Command::new("git")
                    .current_dir(&expanded_path)
                    .arg("branch")
                    .arg("-a")
                    .output()
                {
                    if branch_output.status.success() {
                        let branch_stdout = String::from_utf8_lossy(&branch_output.stdout);
                        for line in branch_stdout.lines() {
                            let mut branch_name = line.trim().replace("*", "").trim().to_string();

                            // リモートブランチの場合、origin/プレフィックスを削除
                            if branch_name.starts_with("remotes/origin/") {
                                branch_name = branch_name
                                    .strip_prefix("remotes/origin/")
                                    .unwrap_or(&branch_name)
                                    .to_string();
                            }

                            // refs/heads/プレフィックスを削除
                            let clean_branch = branch_name
                                .strip_prefix("refs/heads/")
                                .unwrap_or(&branch_name);

                            if !clean_branch.is_empty() && clean_branch != "HEAD" {
                                branches.push(clean_branch.to_string());
                            }
                        }
                    }
                }
            }

            // 重複を除去
            branches.sort();
            branches.dedup();

            if branches.is_empty() {
                branches.push("main".to_string());
            }

            branches
        }
        Err(_) => {
            vec!["main".to_string()]
        }
    }
}

/// 現在の作業ディレクトリのgit worktreeのbranchを取得する関数（後方互換性のため）
pub fn get_git_worktree_branches() -> Vec<String> {
    let current_dir = std::env::current_dir()
        .unwrap_or_else(|_| std::path::PathBuf::from("."))
        .to_string_lossy()
        .to_string();
    get_git_worktree_branches_in_directory(&current_dir)
}

/// 指定されたディレクトリの現在のbranchを取得
pub fn get_current_branch_in_directory(directory: &str) -> String {
    let expanded_path = crate::utils::expand_path(directory);

    if !is_git_repository(&expanded_path) {
        return "main".to_string();
    }

    match Command::new("git")
        .current_dir(&expanded_path)
        .arg("rev-parse")
        .arg("--abbrev-ref")
        .arg("HEAD")
        .output()
    {
        Ok(output) => {
            if output.status.success() {
                let stdout = String::from_utf8_lossy(&output.stdout);
                stdout.trim().to_string()
            } else {
                "main".to_string()
            }
        }
        Err(_) => "main".to_string(),
    }
}

/// 現在のbranchを取得（後方互換性のため）
pub fn get_current_branch() -> String {
    let current_dir = std::env::current_dir()
        .unwrap_or_else(|_| std::path::PathBuf::from("."))
        .to_string_lossy()
        .to_string();
    get_current_branch_in_directory(&current_dir)
}

/// 現在のbranchを取得（Result版）
#[allow(dead_code)]
pub fn get_current_branch_result() -> Result<String, std::io::Error> {
    let current_dir = std::env::current_dir()
        .unwrap_or_else(|_| std::path::PathBuf::from("."))
        .to_string_lossy()
        .to_string();
    Ok(get_current_branch_in_directory(&current_dir))
}

/// 指定されたbranchのworktreeパスを取得
pub fn get_worktree_path(branch: &str) -> Result<String, std::io::Error> {
    let current_dir = std::env::current_dir()
        .unwrap_or_else(|_| std::path::PathBuf::from("."))
        .to_string_lossy()
        .to_string();

    let expanded_path = crate::utils::expand_path(&current_dir);
    let worktree_path = format!("{expanded_path}/claude-schedular-{branch}");

    // worktreeが存在するか確認
    if std::path::Path::new(&worktree_path).exists() {
        Ok(worktree_path)
    } else {
        // worktreeが存在しない場合は現在のディレクトリを返す
        Ok(current_dir)
    }
}

/// git worktreeでのコマンド実行
pub fn execute_command_in_worktree(
    command: &str,
    branch: &str,
    is_shell_mode: bool,
    execution_path: &str,
    claude_skip_permissions: bool,
    claude_continue_from_last: bool,
) -> Result<std::process::Output, std::io::Error> {
    let expanded_path = crate::utils::expand_path(execution_path);
    let worktree_path = format!("{expanded_path}/claude-schedular-{branch}");

    // 指定されたパスが存在するか確認
    if !std::path::Path::new(&expanded_path).exists() {
        return Err(std::io::Error::new(
            std::io::ErrorKind::NotFound,
            format!("指定されたディレクトリが見つかりません: {expanded_path}"),
        ));
    }

    // 指定されたディレクトリがgitリポジトリかチェック
    if !is_git_repository(&expanded_path) {
        return Err(std::io::Error::new(
            std::io::ErrorKind::InvalidData,
            format!("指定されたディレクトリはgitリポジトリではありません: {expanded_path}"),
        ));
    }

    // worktreeが存在しない場合は作成
    let _worktree_create = Command::new("git")
        .current_dir(&expanded_path)
        .arg("worktree")
        .arg("add")
        .arg(format!("claude-schedular-{branch}"))
        .arg(branch)
        .output();

    let full_command = if is_shell_mode {
        command.to_string()
    } else {
        let mut claude_cmd = String::from("claude");
        if claude_skip_permissions {
            claude_cmd.push_str(" --dangerously-skip-permissions");
        }
        if claude_continue_from_last {
            claude_cmd.push_str(" -c");
        }
        format!("{claude_cmd} -p \"{command}\"")
    };

    Command::new("sh")
        .current_dir(&worktree_path)
        .arg("-c")
        .arg(&full_command)
        .output()
}
