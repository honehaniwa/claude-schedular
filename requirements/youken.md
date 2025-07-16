# Claude Scheduler 要件定義書

## 1. プロジェクト概要

### 1.1 背景・目的
- Claude AI コマンドとシェルコマンドをスケジュール実行できるWebベースのアプリケーション
- Git Worktree機能による並列実行サポート
- 時間指定による自動実行とリアルタイム監視
- 実行履歴の管理と再利用

### 1.2 アプリケーション名
`Claude Scheduler` (claude-scheduler)

### 1.3 技術スタック
- **言語**: Rust 2021 Edition
- **UIフレームワーク**: Dioxus 0.5 (Desktop)
- **非同期ランタイム**: Tokio
- **日時処理**: Chrono
- **アーキテクチャ**: モジュラー設計

## 2. 実装済み機能

### 2.1 基本機能 ✅

#### 2.1.1 Claude Code実行
- Claude AIにプロンプトを送信して実行
- 実行結果の表示（stdout/stderr）
- リアルタイム実行状況表示

#### 2.1.2 シェルモード実行
- シェルコマンドの直接実行
- Shell Mode切り替え機能
- コマンドの非同期実行

#### 2.1.3 時間指定スケジュール
- 今日/明日の選択
- 時間（0-23時）と分（0-59分）の設定
- 5秒間隔での自動監視

### 2.2 Git Worktree機能 ✅

#### 2.2.1 並列実行
- 異なるgitブランチでの同時実行
- 利用可能なworktreeブランチの自動検出
- ブランチ選択UI（ドロップダウン）

#### 2.2.2 Worktree管理
- 現在のブランチの自動取得
- Worktreeの存在確認
- ブランチリストの更新機能

### 2.3 スケジュール管理 ✅

#### 2.3.1 スケジュール登録
- コマンド内容の保存
- 実行時刻の設定
- 実行モード（Claude/Shell）の記録
- 対象ブランチの記録

#### 2.3.2 状態管理
- 待機中（Pending）
- 完了（Completed）
- 失敗（Failed）
- 状態の自動更新

### 2.4 実行履歴・結果 ✅

#### 2.4.1 実行履歴
- 全ての実行記録
- 実行時刻、コマンド、結果の保存
- 実行タイプの記録（手動/自動/シェル）

#### 2.4.2 結果表示
- 成功/失敗の色分け表示
- 標準出力/エラー出力の表示
- 実行時間の記録

### 2.5 UI/UX機能 ✅

#### 2.5.1 テーマ機能
- ダークモード/ライトモードの切り替え
- テーマに応じた色の自動調整

#### 2.5.2 レスポンシブUI
- 画面サイズに応じたレイアウト調整
- 使いやすいインターフェース

## 3. データ構造

### 3.1 Schedule構造体
```rust
pub struct Schedule {
    pub id: String,
    pub command: String,
    pub scheduled_time: Option<String>,
    pub _memo: String,
    pub created_at: String,
    pub status: ScheduleStatus,
    pub is_shell_mode: bool,
    pub branch: String,
}
```

### 3.2 ExecutionHistory構造体
```rust
pub struct ExecutionHistory {
    pub id: String,
    pub command: String,
    pub executed_at: String,
    pub execution_type: ExecutionType,
    pub status: ExecutionStatus,
    pub output: String,
    pub branch: String,
}
```

### 3.3 列挙型
```rust
pub enum ScheduleStatus {
    Pending,
    Completed,
    Failed,
}

pub enum ExecutionType {
    Manual,
    Auto,
    FromSchedule,
    ShellMode,
}

pub enum ExecutionStatus {
    Success,
    Failed,
}
```

## 4. ファイル構成

### 4.1 モジュール構成
```
src/
├── main.rs              # エントリーポイント
├── models.rs            # データ構造定義
├── utils.rs             # 時間処理ユーティリティ
├── git.rs               # Git Worktree機能
└── components.rs        # UI コンポーネント
```

### 4.2 主要機能の実装場所
- **スケジュール監視**: `components.rs` の `schedule_checker()`
- **コマンド実行**: `components.rs` の `execute_command`
- **Git Worktree操作**: `git.rs`
- **時間処理**: `utils.rs`

## 5. 実行環境・依存関係

### 5.1 必要環境
- Rust 1.70以上
- Git（worktree機能用）
- Claude CLI（Claude AIモード用）

### 5.2 依存クレート
```toml
[dependencies]
dioxus = { version = "0.5", features = ["desktop"] }
tokio = { version = "1.0", features = ["full"] }
chrono = { version = "0.4", features = ["serde"] }
```

## 6. CLIモード要件

### 6.1 概要
- コマンドラインインターフェースからの操作をサポート
- デスクトップUIなしでスケジューラー機能を利用可能
- バックグラウンドデーモンとしての実行サポート

### 6.2 CLIコマンド仕様

#### 6.2.1 基本コマンド
```bash
# 即座実行
claude-scheduler exec [OPTIONS] <COMMAND>

# スケジュール登録
claude-scheduler schedule [OPTIONS] <COMMAND>

# スケジュール一覧表示
claude-scheduler list [OPTIONS]

# 実行履歴表示
claude-scheduler history [OPTIONS]

# デーモン起動
claude-scheduler daemon [OPTIONS]

# 設定管理
claude-scheduler config [OPTIONS]
```

#### 6.2.2 execサブコマンド
```bash
claude-scheduler exec [OPTIONS] <COMMAND>

OPTIONS:
  -m, --mode <MODE>        実行モード [claude|shell] (default: claude)
  -b, --branch <BRANCH>    Git worktreeブランチ指定
  -w, --worktree          Git worktree並列実行を有効化
  -v, --verbose           詳細出力
  -h, --help              ヘルプ表示
```

#### 6.2.3 scheduleサブコマンド
```bash
claude-scheduler schedule [OPTIONS] <COMMAND>

OPTIONS:
  -t, --time <TIME>       実行時刻 (HH:MM形式)
  -d, --date <DATE>       実行日 [today|tomorrow|YYYY-MM-DD]
  -m, --mode <MODE>       実行モード [claude|shell]
  -b, --branch <BRANCH>   Git worktreeブランチ指定
  -w, --worktree         Git worktree並列実行を有効化
  --memo <MEMO>          メモ追加
  -h, --help             ヘルプ表示
```

#### 6.2.4 listサブコマンド
```bash
claude-scheduler list [OPTIONS]

OPTIONS:
  -s, --status <STATUS>   ステータスでフィルタ [pending|completed|failed]
  -f, --format <FORMAT>   出力形式 [table|json|csv] (default: table)
  -n, --limit <NUMBER>    表示件数制限
  -h, --help             ヘルプ表示
```

#### 6.2.5 historyサブコマンド
```bash
claude-scheduler history [OPTIONS]

OPTIONS:
  -s, --status <STATUS>   ステータスでフィルタ [success|failed]
  -t, --type <TYPE>      実行タイプでフィルタ [manual|auto|shell]
  -b, --branch <BRANCH>   ブランチでフィルタ
  -f, --format <FORMAT>   出力形式 [table|json|csv]
  -n, --limit <NUMBER>    表示件数制限
  --from <DATE>          開始日
  --to <DATE>            終了日
  -h, --help             ヘルプ表示
```

#### 6.2.6 daemonサブコマンド
```bash
claude-scheduler daemon [OPTIONS]

OPTIONS:
  -p, --port <PORT>      APIポート番号 (default: 8080)
  -i, --interval <SEC>   監視間隔（秒） (default: 5)
  --pid-file <PATH>      PIDファイルパス
  --log-file <PATH>      ログファイルパス
  -d, --detach           バックグラウンド実行
  -h, --help             ヘルプ表示
```

### 6.3 データ永続化

#### 6.3.1 設定ファイル
- 場所: `~/.config/claude-scheduler/config.toml`
- 形式: TOML

```toml
[general]
default_mode = "claude"
check_interval = 5

[git]
enable_worktree = false
default_branch = "main"

[storage]
database_path = "~/.local/share/claude-scheduler/db.sqlite"
```

#### 6.3.2 データベース
- SQLite3を使用
- スキーマ:
  - schedules テーブル
  - execution_history テーブル
  - configuration テーブル

### 6.4 実装要件

#### 6.4.1 引数パーサー
- `clap`クレートを使用
- サブコマンド構造の実装
- 引数バリデーション

#### 6.4.2 出力形式
- テーブル形式（人間が読みやすい）
- JSON形式（プログラム連携用）
- CSV形式（データ分析用）

#### 6.4.3 エラーハンドリング
- 終了コードの適切な設定
- エラーメッセージの標準エラー出力
- デバッグ情報のオプショナル出力

#### 6.4.4 デーモンモード
- systemdサービスファイルのサンプル提供
- シグナルハンドリング（SIGTERM, SIGINT）
- グレースフルシャットダウン

### 6.5 互換性要件
- 既存のデスクトップUIとデータ共有
- 同一の実行エンジン使用
- 設定の共有と同期

## 7. 使用方法

### 7.1 基本的な使用手順
1. アプリケーションを起動
2. プロンプトまたはコマンドを入力
3. 実行モード（Claude/Shell）を選択
4. 即座実行またはスケジュール実行を選択
5. 実行結果と履歴を確認

### 7.2 Git Worktree使用手順
1. "Git Worktree並列実行" を有効化
2. 対象ブランチを選択
3. コマンドを実行（選択したブランチのworktreeで実行）

### 7.3 スケジュール実行手順
1. "時間指定自動実行を有効にする" をチェック
2. 実行日（今日/明日）を選択
3. 実行時刻を設定
4. "スケジュール登録" をクリック
5. 5秒間隔で自動実行を監視

---

**更新日**: 2025/07/17
**ステータス**: 実装完了・運用中（GUI + CLI対応）