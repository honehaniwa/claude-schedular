# Claude Code スケジューラー 詳細要件定義

## 1. プロジェクト概要

### 1.1 背景・目的
- Claude Code Proプランのトークン制限（利用開始から5時間後にリセット）対応
- トークン復活タイミングでの自動実行による効率化
- 手動監視の負担軽減とタイミング逃しの防止

### 1.2 アプリケーション名
`Claude Scheduler` (claude-scheduler)

## 2. 機能要件

### 2.1 コア機能

#### 2.1.1 スケジュール管理
```
- 実行予定リスト表示（TreeView形式）
- カラム: ID, コマンド概要, 実行日時, 状態, 優先度
- ソート機能（日時、状態、優先度別）
- フィルタ機能（状態別、日付範囲別）
```

#### 2.1.2 コマンド登録
```
入力フィールド:
- コマンド内容: `claude code -p "プロンプト内容"`
- 実行日時: ユーザーが任意に設定可能（DatePicker + TimePicker）
- 実行タイプ: [即座実行/時間指定/繰り返し]
- 優先度: [高/中/低]
- タグ: カテゴリ分類用
- メモ: 実行目的・注意事項

※重要: トークンリセットタイミング（5時間後）に関わらず、
       ユーザーが指定した任意の時間でコマンドを実行する
```

#### 2.1.3 実行制御
```
状態管理:
- PENDING: 実行待機中
- RUNNING: 実行中
- COMPLETED: 実行完了
- FAILED: 実行失敗
- CANCELLED: ユーザー中止
- RETRY: 再実行待機中

操作:
- 即座実行ボタン
- 実行停止ボタン（SIGTERM -> SIGKILL）
- 削除/編集ボタン
- 一括操作（選択項目の削除/実行）
```

### 2.2 高度機能

#### 2.2.1 バッチ実行機能
```
複数コマンド管理:
- バッチグループ作成
- 実行順序指定（直列/並列）
- 依存関係設定（前のコマンド成功時のみ）
- グループ単位での操作

並列実行制御:
- 最大同時実行数設定（デフォルト: 3）
- CPU/メモリ使用率監視
- リソース不足時の実行待機
```

#### 2.2.2 繰り返し実行
```
繰り返しパターン:
- 毎分/毎時/毎日/毎週/毎月
- cron式対応（* * * * *）
- カスタム間隔（N分後、N時間後）
- 終了条件（回数制限、期限設定）
```

#### 2.2.3 条件付き実行
```
実行条件:
- Claude Code利用可能性チェック
- ネットワーク接続確認
```

### 2.3 Claude Code連携

#### 2.3.1 状態監視
```
監視項目:
- トークン使用状況（5時間サイクル監視）
- トークンリセット予定時刻の自動計算
- API接続状態
- 認証状態
- レスポンス時間
- エラー率統計（429エラー等の監視）
```

#### 2.3.2 実行制御
```
コマンド実行:
- Shellコマンド実行 (std::process::Command使用)
- 標準出力/エラー出力の完全キャプチャ
- 実行結果の詳細ログ保存
- リアルタイム出力表示
- タイムアウト設定（デフォルト: 300秒）
- 文字エンコーディング対応（UTF-8）
- 実行ログの自動ローテーション
```

## 3. UI/UX設計

### 3.1 メインウィンドウ構成
```
レイアウト:
┌─────────────────────────────────────┐
│ メニューバー [ファイル][編集][表示][ヘルプ] │
├─────────────────────────────────────│
│ ツールバー [追加][実行][停止][削除][更新]    │
├─────────────────────────────────────│
│ タブエリア                          │
│ ┌─[スケジュール]─[履歴]─[設定]─────┐│
│ │ スケジュール一覧表示エリア        ││
│ │ (TreeView + スクロールバー)      ││
│ └─────────────────────────────────┘│
├─────────────────────────────────────│
│ 詳細パネル（選択項目の詳細表示）      │
├─────────────────────────────────────│
│ ステータスバー [状態][進行中:N件][次回実行:時刻] │
└─────────────────────────────────────┘
```

### 3.2 ダイアログ設計

#### 3.2.1 コマンド追加/編集ダイアログ
```
フィールド配置:
┌─────────────────────────────────────┐
│ コマンド内容 [                    ] │
│ ┌─実行タイミング─────────────────┐ │
│ │ ○ 即座実行 ○ 時間指定 ○ 繰り返し │ │
│ │ 日時: [YYYY/MM/DD] [HH:MM:SS]  │ │
│ └─────────────────────────────────┘ │
│ 優先度: [高▼] タグ: [          ]   │
│ メモ: [                          ] │
│ [OK] [キャンセル] [テスト実行]      │
└─────────────────────────────────────┘
```

### 3.3 リアルタイム更新
```
更新頻度:
- スケジュール一覧: 1秒間隔
- 実行中コマンド出力: リアルタイム
- システム状態: 5秒間隔
- Claude Code接続状態: 30秒間隔
```

## 4. データ設計

### 4.1 データ管理設計

#### 4.1.1 スケジュールデータ（SQLite）
```sql
-- スケジュールテーブル
CREATE TABLE schedules (
    id TEXT PRIMARY KEY,
    command TEXT NOT NULL,
    scheduled_time DATETIME NOT NULL,
    status TEXT DEFAULT 'PENDING',
    priority INTEGER DEFAULT 2,
    tags TEXT,
    memo TEXT,
    created_at DATETIME DEFAULT CURRENT_TIMESTAMP,
    updated_at DATETIME DEFAULT CURRENT_TIMESTAMP,
    retry_count INTEGER DEFAULT 0,
    max_retries INTEGER DEFAULT 0
);

CREATE INDEX idx_schedules_scheduled_time ON schedules(scheduled_time);
CREATE INDEX idx_schedules_status ON schedules(status);
```

#### 4.1.2 実行履歴（JSON形式）
```json
// logs/execution_history.json
{
  "executions": [
    {
      "id": "uuid-string",
      "schedule_id": "schedule-uuid",
      "command": "claude code -p \"prompt\"",
      "executed_at": "2024-01-01T10:00:00Z",
      "status": "COMPLETED",
      "exit_code": 0,
      "execution_time": 45.5,
      "stdout_log_file": "logs/stdout/2024-01-01_10-00-00_uuid.log",
      "stderr_log_file": "logs/stderr/2024-01-01_10-00-00_uuid.log"
    }
  ]
}
```

#### 4.1.3 実行ログファイル構造
```
logs/
├── execution_history.json     # 実行履歴のメタデータ
├── stdout/                    # 標準出力ログ
│   ├── 2024-01-01_10-00-00_uuid.log
│   └── ...
├── stderr/                    # エラー出力ログ
│   ├── 2024-01-01_10-00-00_uuid.log
│   └── ...
└── app.log                   # アプリケーションログ
```

### 4.2 設定ファイル構造（JSON）
```json
{
    "app": {
        "theme": "light",
        "language": "ja",
        "auto_start": false,
        "minimize_to_tray": true,
        "check_interval": 60
    },
    "execution": {
        "max_parallel": 3,
        "default_timeout": 300,
        "retry_delay": 60,
        "log_retention_days": 30
    },
    "notifications": {
        "desktop_notifications": true,
        "sound_alerts": false,
        "email_alerts": false,
        "slack_webhook": ""
    },
    "claude_code": {
        "command_prefix": "claude code -p",
        "health_check_interval": 30,
        "rate_limit_delay": 1.0,
        "token_reset_interval": 18000,
        "reset_buffer_minutes": 5
    }
}
```

## 5. Rust実装技術仕様

### 5.1 技術スタック選定

#### 5.1.1 GUI フレームワーク
```
推奨: iced (https://github.com/iced-rs/iced)
理由:
- Elm-like アーキテクチャによる予測可能な状態管理
- クロスプラットフォーム対応（Windows, macOS, Linux）
- 豊富なウィジェット（DatePicker, TimePicker等）
- 非同期処理との親和性
- アクティブな開発コミュニティ

代替案:
- egui: 即座モードGUI、シンプル
- tauri: WebベースUI、高度なカスタマイズ可能
- fltk-rs: 軽量、安定性重視
```

#### 5.1.2 依存クレート
```toml
[dependencies]
# GUI フレームワーク
iced = { version = "0.10", features = ["tokio", "debug"] }
iced_aw = "0.7"  # 追加ウィジェット（DatePicker等）

# 非同期ランタイム
tokio = { version = "1.0", features = ["full"] }

# データベース
sqlx = { version = "0.7", features = ["runtime-tokio-rustls", "sqlite", "chrono", "uuid"] }

# シリアライゼーション
serde = { version = "1.0", features = ["derive"] }
serde_json = "1.0"
toml = "0.8"

# 日時処理
chrono = { version = "0.4", features = ["serde"] }

# ログ
tracing = "0.1"
tracing-subscriber = { version = "0.3", features = ["env-filter"] }

# エラーハンドリング
anyhow = "1.0"
thiserror = "1.0"

# プロセス実行
tokio-process = "0.2"

# 通知
notify-rust = "4.0"

# UUID生成
uuid = { version = "1.0", features = ["v4", "serde"] }

# 設定管理
config = "0.13"
directories = "5.0"
```

### 5.2 アーキテクチャ設計

#### 5.2.1 Clean Architecture適用
```
src/
├── main.rs                    # エントリーポイント
├── app/                       # アプリケーション層
│   ├── mod.rs
│   ├── scheduler_service.rs   # スケジューラーサービス
│   ├── executor_service.rs    # 実行サービス
│   └── notification_service.rs # 通知サービス
├── domain/                    # ドメイン層
│   ├── mod.rs
│   ├── entities/              # エンティティ
│   │   ├── mod.rs
│   │   ├── schedule.rs
│   │   └── execution_result.rs
│   ├── value_objects/         # 値オブジェクト
│   │   ├── mod.rs
│   │   ├── command.rs
│   │   └── schedule_time.rs
│   └── repositories/          # リポジトリトレイト
│       ├── mod.rs
│       └── schedule_repository.rs
├── infrastructure/            # インフラストラクチャ層
│   ├── mod.rs
│   ├── database/
│   │   ├── mod.rs
│   │   ├── sqlite_repository.rs
│   │   └── migrations/
│   ├── config/
│   │   ├── mod.rs
│   │   └── app_config.rs
│   └── process/
│       ├── mod.rs
│       └── command_executor.rs
└── presentation/              # プレゼンテーション層
    ├── mod.rs
    ├── gui/
    │   ├── mod.rs
    │   ├── main_window.rs
    │   ├── add_schedule_dialog.rs
    │   ├── widgets/
    │   │   ├── mod.rs
    │   │   ├── schedule_list.rs
    │   │   └── date_time_picker.rs
    │   └── styles/
    │       ├── mod.rs
    │       └── theme.rs
    └── messages/
        ├── mod.rs
        └── app_message.rs
```

#### 5.2.2 iced アーキテクチャ
```rust
// メインアプリケーション構造体
#[derive(Debug)]
pub struct ClaudeSchedulerApp {
    schedules: Vec<Schedule>,
    selected_schedule: Option<usize>,
    show_add_dialog: bool,
    scheduler_service: Arc<SchedulerService>,
}

// アプリケーションメッセージ
#[derive(Debug, Clone)]
pub enum Message {
    // スケジュール管理
    AddSchedule(Schedule),
    UpdateSchedule(usize, Schedule),
    DeleteSchedule(usize),
    SelectSchedule(usize),
    
    // 実行制御
    ExecuteNow(usize),
    CancelExecution(usize),
    
    // UI操作
    ShowAddDialog,
    HideAddDialog,
    RefreshList,
    
    // システム
    Tick,
    ExecutionCompleted(ExecutionResult),
}

// アプリケーション実装
impl Application for ClaudeSchedulerApp {
    type Message = Message;
    type Theme = iced::Theme;
    type Executor = iced::executor::Default;
    type Flags = ();

    fn new(_flags: Self::Flags) -> (Self, Command<Self::Message>) {
        // 初期化処理
    }

    fn title(&self) -> String {
        "Claude Scheduler".to_string()
    }

    fn update(&mut self, message: Self::Message) -> Command<Self::Message> {
        // メッセージ処理
    }

    fn view(&self) -> Element<Self::Message> {
        // UI構築
    }

    fn subscription(&self) -> Subscription<Self::Message> {
        // 定期処理（1秒ごとのTick等）
    }
}
```

### 5.3 非同期処理設計

#### 5.3.1 tokio による並行処理
```rust
use tokio::{sync::mpsc, task::JoinHandle};

// チャンネル設計
pub struct Channels {
    // 実行要求チャンネル
    execution_tx: mpsc::UnboundedSender<ExecutionRequest>,
    execution_rx: mpsc::UnboundedReceiver<ExecutionRequest>,
    
    // 実行結果チャンネル
    result_tx: mpsc::UnboundedSender<ExecutionResult>,
    result_rx: mpsc::UnboundedReceiver<ExecutionResult>,
    
    // GUI更新チャンネル
    gui_tx: mpsc::UnboundedSender<GuiUpdate>,
    gui_rx: mpsc::UnboundedReceiver<GuiUpdate>,
}

// バックグラウンドタスク
pub struct BackgroundTasks {
    scheduler_handle: JoinHandle<()>,
    executor_handles: Vec<JoinHandle<()>>,
    notification_handle: JoinHandle<()>,
}

impl BackgroundTasks {
    pub async fn start(channels: Channels) -> Self {
        let scheduler_handle = tokio::spawn(Self::scheduler_loop(/* ... */));
        let executor_handles = (0..3)
            .map(|_| tokio::spawn(Self::executor_loop(/* ... */)))
            .collect();
        let notification_handle = tokio::spawn(Self::notification_loop(/* ... */));
        
        Self {
            scheduler_handle,
            executor_handles,
            notification_handle,
        }
    }
    
    async fn scheduler_loop(/* ... */) {
        // スケジュール監視ループ
        let mut interval = tokio::time::interval(Duration::from_secs(1));
        loop {
            interval.tick().await;
            // 実行時刻チェック・実行要求送信
        }
    }
    
    async fn executor_loop(/* ... */) {
        // コマンド実行ループ
        while let Some(request) = execution_rx.recv().await {
            // claude code コマンド実行
            let result = execute_command(&request.command).await;
            result_tx.send(result).unwrap();
        }
    }
}
```

### 5.4 データ管理実装（Rust）

#### 5.4.1 スケジュールデータ（SQLx使用）
```sql
-- migrations/001_create_schedules.sql
CREATE TABLE schedules (
    id TEXT PRIMARY KEY,
    command TEXT NOT NULL,
    scheduled_time DATETIME NOT NULL,
    status TEXT NOT NULL DEFAULT 'PENDING',
    priority INTEGER NOT NULL DEFAULT 2,
    tags TEXT,
    memo TEXT,
    created_at DATETIME NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at DATETIME NOT NULL DEFAULT CURRENT_TIMESTAMP,
    retry_count INTEGER NOT NULL DEFAULT 0,
    max_retries INTEGER NOT NULL DEFAULT 0
);

CREATE INDEX idx_schedules_scheduled_time ON schedules(scheduled_time);
CREATE INDEX idx_schedules_status ON schedules(status);
```

#### 5.4.2 実行履歴管理（JSON）
```rust
use serde::{Deserialize, Serialize};
use std::fs;
use std::path::PathBuf;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ExecutionHistory {
    pub executions: Vec<ExecutionRecord>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ExecutionRecord {
    pub id: String,
    pub schedule_id: Option<String>,
    pub command: String,
    pub executed_at: chrono::DateTime<chrono::Utc>,
    pub status: String,
    pub exit_code: Option<i32>,
    pub execution_time: f64,
    pub stdout_log_file: Option<PathBuf>,
    pub stderr_log_file: Option<PathBuf>,
}

impl ExecutionHistory {
    pub async fn load_from_file(path: &PathBuf) -> Result<Self> {
        match fs::read_to_string(path) {
            Ok(content) => Ok(serde_json::from_str(&content)?),
            Err(_) => Ok(ExecutionHistory { executions: vec![] }),
        }
    }
    
    pub async fn save_to_file(&self, path: &PathBuf) -> Result<()> {
        let content = serde_json::to_string_pretty(self)?;
        fs::write(path, content)?;
        Ok(())
    }
    
    pub fn add_execution(&mut self, record: ExecutionRecord) {
        self.executions.push(record);
    }
}
```

#### 5.4.3 シェルコマンド実行
```rust
use std::process::{Command, Stdio};
use std::io::{BufRead, BufReader};
use tokio::fs::File;
use tokio::io::AsyncWriteExt;

pub struct CommandExecutor {
    log_dir: PathBuf,
}

impl CommandExecutor {
    pub async fn execute_command(&self, cmd: &str, schedule_id: &str) -> Result<ExecutionRecord> {
        let start_time = chrono::Utc::now();
        let execution_id = uuid::Uuid::new_v4().to_string();
        
        // ログファイルパス生成
        let timestamp = start_time.format("%Y-%m-%d_%H-%M-%S");
        let stdout_path = self.log_dir.join("stdout").join(format!("{}_{}.log", timestamp, execution_id));
        let stderr_path = self.log_dir.join("stderr").join(format!("{}_{}.log", timestamp, execution_id));
        
        // ディレクトリ作成
        tokio::fs::create_dir_all(stdout_path.parent().unwrap()).await?;
        tokio::fs::create_dir_all(stderr_path.parent().unwrap()).await?;
        
        // コマンド実行
        let mut child = Command::new("sh")
            .arg("-c")
            .arg(cmd)
            .stdout(Stdio::piped())
            .stderr(Stdio::piped())
            .spawn()?;
        
        // 出力キャプチャとログ保存
        let stdout_handle = self.capture_output(child.stdout.take().unwrap(), stdout_path.clone()).await;
        let stderr_handle = self.capture_output(child.stderr.take().unwrap(), stderr_path.clone()).await;
        
        // プロセス終了待機
        let exit_status = child.wait()?;
        let end_time = chrono::Utc::now();
        let execution_time = (end_time - start_time).num_milliseconds() as f64 / 1000.0;
        
        Ok(ExecutionRecord {
            id: execution_id,
            schedule_id: Some(schedule_id.to_string()),
            command: cmd.to_string(),
            executed_at: start_time,
            status: if exit_status.success() { "COMPLETED".to_string() } else { "FAILED".to_string() },
            exit_code: exit_status.code(),
            execution_time,
            stdout_log_file: Some(stdout_path),
            stderr_log_file: Some(stderr_path),
        })
    }
    
    async fn capture_output<R: std::io::Read + Send + 'static>(
        &self, 
        reader: R, 
        log_path: PathBuf
    ) -> tokio::task::JoinHandle<Result<()>> {
        tokio::task::spawn(async move {
            let mut file = File::create(&log_path).await?;
            let mut buf_reader = BufReader::new(reader);
            let mut line = String::new();
            
            while buf_reader.read_line(&mut line)? > 0 {
                file.write_all(line.as_bytes()).await?;
                line.clear();
            }
            
            file.flush().await?;
            Ok(())
        })
    }
}
```

#### 5.4.4 リポジトリ実装
```rust
use sqlx::{SqlitePool, Row};
use uuid::Uuid;
use async_trait::async_trait;

#[async_trait]
pub trait ScheduleRepository: Send + Sync {
    async fn save(&self, schedule: &Schedule) -> Result<()>;
    async fn find_by_id(&self, id: &Uuid) -> Result<Option<Schedule>>;
    async fn find_pending(&self) -> Result<Vec<Schedule>>;
    async fn update_status(&self, id: &Uuid, status: ScheduleStatus) -> Result<()>;
    async fn delete(&self, id: &Uuid) -> Result<()>;
}

pub struct SqliteScheduleRepository {
    pool: SqlitePool,
}

#[async_trait]
impl ScheduleRepository for SqliteScheduleRepository {
    async fn save(&self, schedule: &Schedule) -> Result<()> {
        sqlx::query!(
            r#"
            INSERT INTO schedules (id, command, scheduled_time, status, priority, tags, memo)
            VALUES (?, ?, ?, ?, ?, ?, ?)
            "#,
            schedule.id.to_string(),
            schedule.command.to_string(),
            schedule.scheduled_time,
            schedule.status.to_string(),
            schedule.priority as i32,
            schedule.tags.join(","),
            schedule.memo
        )
        .execute(&self.pool)
        .await?;
        
        Ok(())
    }
    
    // 他のメソッドも実装...
}
```

### 5.5 エラーハンドリング

#### 5.5.1 カスタムエラー型
```rust
use thiserror::Error;

#[derive(Error, Debug)]
pub enum SchedulerError {
    #[error("Database error: {0}")]
    Database(#[from] sqlx::Error),
    
    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),
    
    #[error("Configuration error: {message}")]
    Configuration { message: String },
    
    #[error("Command execution failed: {command}")]
    CommandExecution { command: String },
    
    #[error("Parse error: {0}")]
    Parse(#[from] chrono::ParseError),
    
    #[error("Serialization error: {0}")]
    Serialization(#[from] serde_json::Error),
}

pub type Result<T> = std::result::Result<T, SchedulerError>;
```

### 5.6 設定管理

#### 5.6.1 設定構造体
```rust
use serde::{Deserialize, Serialize};
use std::path::PathBuf;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct AppConfig {
    pub app: AppSettings,
    pub execution: ExecutionSettings,
    pub notifications: NotificationSettings,
    pub claude_code: ClaudeCodeSettings,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct AppSettings {
    pub theme: String,
    pub language: String,
    pub auto_start: bool,
    pub minimize_to_tray: bool,
    pub check_interval: u64,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ExecutionSettings {
    pub max_parallel: usize,
    pub default_timeout: u64,
    pub retry_delay: u64,
    pub log_retention_days: u32,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ClaudeCodeSettings {
    pub command_prefix: String,
    pub health_check_interval: u64,
    pub rate_limit_delay: f64,
    pub token_reset_interval: u64,
    pub reset_buffer_minutes: u64,
}

impl Default for AppConfig {
    fn default() -> Self {
        Self {
            app: AppSettings {
                theme: "light".to_string(),
                language: "ja".to_string(),
                auto_start: false,
                minimize_to_tray: true,
                check_interval: 60,
            },
            execution: ExecutionSettings {
                max_parallel: 3,
                default_timeout: 300,
                retry_delay: 60,
                log_retention_days: 30,
            },
            notifications: NotificationSettings {
                desktop_notifications: true,
                sound_alerts: false,
                email_alerts: false,
                slack_webhook: None,
            },
            claude_code: ClaudeCodeSettings {
                command_prefix: "claude code -p".to_string(),
                health_check_interval: 30,
                rate_limit_delay: 1.0,
                token_reset_interval: 18000, // 5時間
                reset_buffer_minutes: 5,
            },
        }
    }
}
```

### 5.7 ビルドとデプロイメント

#### 5.7.1 Cargo.toml 設定
```toml
[package]
name = "claude-scheduler"
version = "0.1.0"
edition = "2021"
authors = ["Your Name <email@example.com>"]
description = "Claude Code command scheduler with GUI"
license = "MIT"

[profile.release]
lto = true
codegen-units = 1
panic = "abort"
strip = true

[target.x86_64-pc-windows-gnu]
linker = "x86_64-w64-mingw32-gcc"

[target.x86_64-apple-darwin]
rustflags = ["-C", "link-arg=-undefined", "-C", "link-arg=dynamic_lookup"]
```

#### 5.7.2 クロスプラットフォームビルド
```bash
# Windows
cargo build --release --target x86_64-pc-windows-gnu

# macOS (Intel)
cargo build --release --target x86_64-apple-darwin

# macOS (Apple Silicon)
cargo build --release --target aarch64-apple-darwin

# Linux
cargo build --release --target x86_64-unknown-linux-gnu
```

### 5.8 テスト戦略

#### 5.8.1 テスト構成
```rust
// tests/integration_tests.rs
use claude_scheduler::*;

#[tokio::test]
async fn test_schedule_creation() {
    let config = AppConfig::default();
    let scheduler = SchedulerService::new(config).await.unwrap();
    
    let schedule = Schedule::new(
        "claude code -p \"test command\"".to_string(),
        chrono::Utc::now() + chrono::Duration::minutes(5),
    );
    
    scheduler.add_schedule(schedule).await.unwrap();
    let schedules = scheduler.get_pending_schedules().await.unwrap();
    assert_eq!(schedules.len(), 1);
}

#[test]
fn test_command_validation() {
    let dangerous_commands = vec![
        "rm -rf /",
        "claude code -p \"test\" && rm file",
        "claude code -p \"test\"; cat /etc/passwd",
    ];
    
    for cmd in dangerous_commands {
        assert!(!validate_command(cmd));
    }
}
```

この Rust 実装仕様により、安全で効率的なクロスプラットフォーム GUI アプリケーションを構築できます。

## 6. エラーハンドリング

### 6.1 エラー分類
```python
class SchedulerError(Exception):
    """基底例外クラス"""

class CommandExecutionError(SchedulerError):
    """コマンド実行エラー"""

class DatabaseError(SchedulerError):
    """データベースエラー"""

class ConfigurationError(SchedulerError):
    """設定エラー"""

class NetworkError(SchedulerError):
    """ネットワークエラー"""
```

### 6.2 リトライ戦略
```python
# 指数バックオフによるリトライ
retry_delays = [1, 2, 4, 8, 16]  # 秒
max_retries = 5

# エラー種別によるリトライ可否
RETRYABLE_ERRORS = {
    'NetworkError': True,
    'TimeoutError': True,
    'CommandExecutionError': False,
    'ConfigurationError': False
}
```

## 7. セキュリティ要件

### 7.1 コマンドインジェクション防止
```python
import shlex
import re

DANGEROUS_PATTERNS = [
    r'[;&|`$(){}<>]',  # シェル特殊文字
    r'\.\./',          # ディレクトリトラバーサル
    r'rm\s+-rf',       # 危険なコマンド
]

def validate_command(command: str) -> bool:
    for pattern in DANGEROUS_PATTERNS:
        if re.search(pattern, command):
            return False
    return True
```

### 7.2 ログの機密情報マスキング
```python
SENSITIVE_PATTERNS = [
    r'password[=:]\s*\S+',
    r'token[=:]\s*\S+',
    r'key[=:]\s*\S+',
    r'secret[=:]\s*\S+'
]

def mask_sensitive_info(text: str) -> str:
    for pattern in SENSITIVE_PATTERNS:
        text = re.sub(pattern, r'\1****', text, flags=re.IGNORECASE)
    return text
```

## 8. パフォーマンス要件

### 8.1 レスポンス時間
- GUI操作応答: < 100ms
- スケジュール追加: < 200ms
- 一覧表示更新: < 500ms
- データベース操作: < 1s

### 8.2 リソース使用量
- メモリ使用量: < 100MB（idle時）
- CPU使用率: < 5%（idle時）
- ディスク容量: < 50MB（アプリ本体）

### 8.3 スケーラビリティ
- 最大登録スケジュール数: 10,000件
- 同時実行コマンド数: 設定可能（1-10）
- 履歴保存期間: 設定可能（1日-1年）

## 9. 運用要件

### 9.1 ログ管理
```
ログレベル:
- DEBUG: 詳細なデバッグ情報
- INFO: 一般的な動作情報
- WARNING: 警告メッセージ
- ERROR: エラー情報
- CRITICAL: 致命的エラー

ログファイル構造:
- logs/app.log: アプリケーションログ
- logs/execution_history.json: 実行履歴メタデータ
- logs/stdout/YYYY-MM-DD_HH-MM-SS_uuid.log: 標準出力ログ
- logs/stderr/YYYY-MM-DD_HH-MM-SS_uuid.log: エラー出力ログ

ログローテーション:
- 実行ログ: 日次ローテーション、30日保持
- アプリケーションログ: サイズベース（10MB）、5世代保持
```

### 9.2 バックアップ・復元
```rust
pub async fn backup_data() -> Result<()> {
    // データベース、設定、実行履歴をバックアップ
    let backup_files = vec![
        "scheduler.db",
        "config.toml", 
        "logs/execution_history.json",
        "logs/stdout/",
        "logs/stderr/"
    ];
    
    // ZIP形式でバックアップファイル作成
    let backup_name = format!("backup_{}.zip", chrono::Utc::now().format("%Y%m%d_%H%M%S"));
    // バックアップ処理実装...
}

pub async fn restore_data(backup_path: &Path) -> Result<()> {
    // バックアップファイルから復元
    // データ整合性チェック後に復元実行
}
```

## 10. 開発・テスト要件

### 10.1 開発環境
- Rust 1.70+
- iced GUI フレームワーク
- SQLite (sqlx)
- その他開発ツール: cargo-watch, cargo-audit, clippy

### 10.2 テスト戦略
```
テスト種別:
- 単体テスト: 各クラス・関数のテスト
- 統合テスト: 複数モジュール間のテスト
- GUI テスト: ユーザーインターフェースのテスト
- 負荷テスト: 大量データ処理のテスト
```

### 10.3 デプロイメント
```
配布形態:
- スタンドアロン実行可能ファイル（cargo build --release）
- GitHub Release (クロスプラットフォームバイナリ)
- Cargo crate (cargo install claude-scheduler)

対応OS:
- Windows 10/11 (x86_64)
- macOS 10.15+ (x86_64, aarch64)
- Linux (Ubuntu 18.04+, x86_64)
```