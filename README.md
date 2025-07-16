# Claude Scheduler

Claude AIコマンドとGit Worktreeに対応したWebベースのスケジューラー - Dioxusで構築

[![CI/CD](https://github.com/honehaniwa/claude-scheduler/actions/workflows/ci.yml/badge.svg)](https://github.com/your-username/claude-scheduler/actions/workflows/ci.yml)
[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)
[![Rust](https://img.shields.io/badge/rust-1.70%2B-blue.svg)](https://www.rust-lang.org/)
[![Dioxus](https://img.shields.io/badge/dioxus-0.5-green.svg)](https://dioxuslabs.com/)
[![codecov](https://codecov.io/gh/your-username/claude-scheduler/branch/main/graph/badge.svg)](https://codecov.io/gh/your-username/claude-scheduler)

## 概要

Claude SchedulerはClaude AIコマンドやシェルコマンドをGUIで簡単にスケジュール実行できるデスクトップアプリケーションです。RustとDioxusフレームワークを使用したモダンなWeb UI技術で構築されています。

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
git clone https://github.com/your-username/claude-scheduler.git
cd claude-scheduler

# プロジェクトをビルド
cargo build --release

# アプリケーションを実行
cargo run
```

### ビルド済みバイナリの使用
[リリースページ](https://github.com/your-username/claude-scheduler/releases)から最新版をダウンロードしてください。

## 使い方

### 1. Claude Codeモード（デフォルト）
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

### 5. 実行履歴の確認
- 「📊 実行履歴・結果」セクションで全ての実行履歴を確認
- 緑色のボーダーは成功、赤色は失敗を示します
- 「🔄 再利用」ボタンで過去のコマンドを再実行可能

## 設定

アプリケーションは実行時に設定と履歴をメモリに保存します。永続化ストレージについては、データベースやファイルシステムを使用するようにコードを修正してください。

## 貢献

貢献を歓迎します！ぜひPull Requestをお送りください。

### 開発環境のセットアップ
```bash
# リポジトリをクローン
git clone https://github.com/your-username/claude-scheduler.git
cd claude-scheduler

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
