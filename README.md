# Claude Scheduler

Claude AIコマンドとGit Worktreeに対応したスケジューラー - CLI & GUIの両方をサポート

[![CI/CD](https://github.com/honehaniwa/claude-schedular/actions/workflows/ci.yml/badge.svg)](https://github.com/honehaniwa/claude-schedular/actions/workflows/ci.yml)
[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)
[![Rust](https://img.shields.io/badge/rust-1.70%2B-blue.svg)](https://www.rust-lang.org/)
[![Dioxus](https://img.shields.io/badge/dioxus-0.5-green.svg)](https://dioxuslabs.com/)

## 概要

Claude SchedulerはClaude AIコマンドやシェルコマンドをスケジュール実行できるツールです。デスクトップGUIとCLIの両方のインターフェースを提供し、RustとDioxusフレームワークで構築されています。

## 主な機能

### ✅ 基本機能
- **Claude Code実行**: Claude AIにプロンプトを送信して実行
- **シェルモード**: シェルコマンドを直接実行
- **時間指定スケジュール**: 今日/明日の任意の時間にスケジュール実行
- **即座実行**: コマンドを即座に実行

### ✅ Git Worktree対応
- **並列実行**: 異なるgitブランチで同時にコマンドを実行
- **ブランチ選択**: 利用可能なgit worktreeブランチから選択
- **Worktree管理**: 自動的なworktreeの検出と管理

### ✅ スケジュール機能
- **分単位の精度**: 正確な時間設定（0-23時、0-59分）
- **スケジュール管理**: 追加、編集、削除、一覧表示
- **5秒間隔監視**: 自動的なスケジュール監視
- **状態管理**: 待機中、完了、失敗の実行状況追跡

### ✅ 実行履歴・結果
- **実行履歴**: 全ての実行を記録（手動、自動、シェル）
- **詳細結果**: stdout/stderrの出力を表示
- **コマンド再利用**: 履歴からコマンドを再利用
- **色分け結果**: 成功/失敗を視覚的に区別

### ✅ UI/UX機能
- **ダークモード**: ライト/ダークテーマの切り替え
- **レスポンシブUI**: 画面サイズに適応
- **リアルタイムステータス**: 実行状況をリアルタイム更新

## インストール

### 必要環境
- Rust 1.70以上
- Git（worktree機能用）
- Claude CLI（Claude AIモード用）

### ソースからビルド
```bash
# リポジトリをクローン
git clone https://github.com/honehaniwa/claude-schedular.git
cd claude-schedular

# プロジェクトをビルド
cargo build --release

# アプリケーションを実行
cargo run
```

### 開発者向け

#### Pre-commitフック
このプロジェクトにはpre-commitフックが設定されており、`git add`する前に自動的に`cargo fmt --all -- --check`が実行されます。

フォーマットエラーがある場合は、以下のコマンドで修正してください：
```bash
cargo fmt --all
```

### ビルド済みバイナリの使用
[リリースページ](https://github.com/honehaniwa/claude-schedular/releases)から最新版をダウンロードしてください。

## 使い方

### GUI モード（デスクトップアプリケーション）

引数なしで実行するとGUIモードが起動します：
```bash
./claude-scheduler
# または
cargo run
```

#### 1. Claude Codeモード（デフォルト）
1. テキストエリアにClaude AIのプロンプトを入力
2. 「▶️ 即座実行」をクリックするか、スケジュール実行を設定

### 2. シェルモード
1. 「💻 Shell Mode」チェックボックスを有効化
2. シェルコマンドを入力（例：`ls -la`、`echo 'hello'`）
3. コマンドを直接実行

### 3. Git Worktreeモード
1. 「🌿 Git Worktree並列実行」を有効化
2. ドロップダウンからターゲットブランチを選択
3. 選択したブランチのworktreeでコマンドが実行されます

### 4. スケジュール実行
1. 「⏰ 時間指定自動実行を有効にする」をチェック
2. 今日/明日を選択
3. 時間（0-23時）と分（0-59分）を設定
4. 「📅 スケジュール登録」をクリック

### CLI モード（コマンドライン）

引数を指定して実行するとCLIモードで動作します：

#### 基本的な使い方

```bash
# ヘルプを表示
./claude-scheduler --help

# 即座実行（Claudeモード）
./claude-scheduler exec "create a Python hello world script"

# シェルコマンドを即座実行
./claude-scheduler exec -m shell "ls -la"

# Git worktreeを使用して実行
./claude-scheduler exec -w -b feature-branch "run tests"

# Claude実行時に確認をスキップ
./claude-scheduler exec --skip-permissions "create a function"

# 前回のClaudeセッションから継続
./claude-scheduler exec -c "continue the implementation"

# 両方のオプションを使用
./claude-scheduler exec --skip-permissions -c "fix the bug"

# スケジュール登録（明日の15:30に実行）
./claude-scheduler schedule "backup database" -t 15:30 -d tomorrow

# スケジュール一覧を表示
./claude-scheduler list

# JSON形式で出力
./claude-scheduler list -f json

# 実行履歴を表示（最新10件）
./claude-scheduler history -n 10

# デーモンとして起動（バックグラウンドでスケジュール監視）
./claude-scheduler daemon
```

#### CLI サブコマンド

##### `exec` - 即座実行
```bash
claude-scheduler exec [OPTIONS] <COMMAND>

OPTIONS:
  -m, --mode <MODE>        実行モード [claude|shell] (default: claude)
  -b, --branch <BRANCH>    Git worktreeブランチ指定
  -w, --worktree          Git worktree並列実行を有効化
  --skip-permissions       Claude実行時の確認をスキップ
  -c, --continue-from-last 前回のClaudeセッションから継続
  -v, --verbose           詳細出力
```

##### `schedule` - スケジュール登録
```bash
claude-scheduler schedule [OPTIONS] <COMMAND>

OPTIONS:
  -t, --time <TIME>       実行時刻 (HH:MM形式)
  -d, --date <DATE>       実行日 [today|tomorrow|YYYY-MM-DD]
  -m, --mode <MODE>       実行モード [claude|shell]
  -b, --branch <BRANCH>   Git worktreeブランチ指定
  -w, --worktree         Git worktree並列実行を有効化
  --memo <MEMO>          メモ追加
  --skip-permissions      Claude実行時の確認をスキップ
  --continue-from-last    前回のClaudeセッションから継続
```

##### `list` - スケジュール一覧
```bash
claude-scheduler list [OPTIONS]

OPTIONS:
  -s, --status <STATUS>   ステータスでフィルタ [pending|completed|failed]
  -f, --format <FORMAT>   出力形式 [table|json|csv] (default: table)
  -n, --limit <NUMBER>    表示件数制限
```

##### `history` - 実行履歴
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
```

##### `daemon` - デーモン起動
```bash
claude-scheduler daemon [OPTIONS]

OPTIONS:
  -p, --port <PORT>      APIポート番号 (default: 8080)
  -i, --interval <SEC>   監視間隔（秒） (default: 5)
  --pid-file <PATH>      PIDファイルパス
  --log-file <PATH>      ログファイルパス
  -d, --detach           バックグラウンド実行
```

##### `config` - 設定管理
```bash
# 全設定を表示
claude-scheduler config show

# 特定の設定値を取得
claude-scheduler config get default_mode

# 設定値を変更
claude-scheduler config set default_mode shell
```

#### データ保存場所

CLIモードでは以下の場所にデータが保存されます：
- **設定ファイル**: `~/.config/claude-scheduler/config.toml`
- **データベース**: `~/.local/share/claude-scheduler/db.sqlite`

### 5. 実行履歴の確認
- 「📊 実行履歴・結果」セクションで全ての実行履歴を確認
- 緑色のボーダーは成功、赤色は失敗を示します
- 「🔄 再利用」ボタンで過去のコマンドを再実行可能

## 設定

### GUI モード
アプリケーションは実行時に設定と履歴をメモリに保存します。

### CLI モード
設定は`~/.config/claude-scheduler/config.toml`に保存されます：

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

## 貢献

貢献を歓迎します！ぜひPull Requestをお送りください。

### 開発環境のセットアップ
```bash
# リポジトリをクローン
git clone https://github.com/honehaniwa/claude-schedular.git
cd claude-schedular

# 依存関係をインストール
cargo build

# 開発モードで実行
cargo run

# テスト実行
cargo test

# コードフォーマット
cargo fmt

# リントチェック
cargo clippy --all-targets --all-features -- -D warnings
```

### CI/CDパイプライン
このプロジェクトには包括的なCI/CDパイプラインが設定されています：

- **自動テスト**: プッシュ・PRごとに全テストを実行
- **コードフォーマット**: `cargo fmt` でフォーマットチェック
- **リントチェック**: `cargo clippy` で品質チェック
- **セキュリティ監査**: `cargo audit` で脆弱性チェック
- **マルチプラットフォームビルド**: Linux/Windows/macOS対応
- **自動バイナリビルド**: リリース用バイナリの自動生成
- **コードカバレッジ**: テストカバレッジの測定と報告
- **依存関係自動更新**: Dependabotによる依存関係の自動更新

### 貢献方法
1. このリポジトリをフォーク
2. 機能ブランチを作成 (`git checkout -b feature/new-feature`)
3. 変更をコミット (`git commit -am 'Add new feature'`)
4. ブランチにプッシュ (`git push origin feature/new-feature`)
5. Pull Requestを作成

詳細は [CONTRIBUTING.md](CONTRIBUTING.md) を参照してください。

### Issue・バグ報告
- [バグレポート](.github/ISSUE_TEMPLATE/bug_report.md) テンプレート
- [機能リクエスト](.github/ISSUE_TEMPLATE/feature_request.md) テンプレート
- [セキュリティポリシー](SECURITY.md) に従った脆弱性報告

## 技術仕様

- **言語**: Rust 2021 Edition
- **UIフレームワーク**: Dioxus 0.5 (Desktop)
- **非同期ランタイム**: Tokio
- **日時処理**: Chrono
- **アーキテクチャ**: 関心事の分離によるモジュラー設計

## ライセンス

このプロジェクトはMITライセンスの下で公開されています。詳細は[LICENSE](LICENSE)ファイルをご覧ください。

## 謝辞

- [Dioxus](https://dioxuslabs.com/) - 優れたUIフレームワーク
- [Claude AI](https://claude.ai/) - AI機能
- Rustコミュニティ - 素晴らしいエコシステム

## サポート

問題が発生した場合やご質問がございましたら、GitHubでIssueを作成してください。

---

[English README](en_README.md) | [English Requirements](requirements/en_requirements.md) 
## CI/CDテスト完了
最終更新: #午後

## Codecov設定手順

このプロジェクトはコードカバレッジ測定にCodecovを使用していますが、現在トークンが設定されていないため無効化されています。有効化するには：

1. **Codecovアカウントの作成**
   - [codecov.io](https://codecov.io/)にアクセス
   - GitHubアカウントでサインイン

2. **リポジトリの追加**
   - Codecovダッシュボードで「Add a repository」をクリック
   - `honehaniwa/claude-schedular`を選択

3. **トークンの取得**
   - リポジトリ設定ページでアップロードトークンをコピー

4. **GitHub Secretsに追加**
   - GitHubリポジトリの Settings > Secrets and variables > Actions
   - 「New repository secret」をクリック
   - Name: `CODECOV_TOKEN`
   - Value: コピーしたトークンを貼り付け

5. **CI設定の更新**
   - `.github/workflows/ci.yml`の該当部分（110-115行目）のコメントを解除：
   ```yaml
   - name: Upload coverage to Codecov
     uses: codecov/codecov-action@v4
     with:
       files: lcov.info
       fail_ci_if_error: true
       token: ${{ secrets.CODECOV_TOKEN }}  # この行を追加
   ```

6. **バッジの追加（オプション）**
   - Codecovダッシュボードからバッジ用のMarkdownをコピー
   - README.mdのバッジセクションに追加：
   ```markdown
   [![codecov](https://codecov.io/gh/honehaniwa/claude-schedular/branch/main/graph/badge.svg?token=YOUR_TOKEN)](https://codecov.io/gh/honehaniwa/claude-schedular)
   ```

---

## ⚠️ セキュリティ監査（cargo audit）について

本プロジェクトは依存クレートの都合により、現時点でcargo auditによるセキュリティチェックがfailします。

- sqlx経由でrsaクレートの脆弱性（Marvin Attack: RUSTSEC-2023-0071）が含まれます（2024年7月現在、修正版なし）
- GUIモードではGTK3系の依存が「メンテされていない」警告を出します
- CLIモードのみでビルド（`cargo build --release --no-default-features`）すれば、これらの影響は実質ありません
- CI/CDではcargo auditジョブはallow failure（失敗しても全体はfailにならない）設定です

依存クレートのアップデートにより将来的に解消される見込みです。
