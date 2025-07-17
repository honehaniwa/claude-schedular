# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [1.0.1] - 2025-01-17

### Added
- GUI版のWindows/macOSサポート（全プラットフォームでGUI版が利用可能に）
- スケジュール編集機能（既存スケジュールの内容を変更可能）
- 実行履歴の永続化（JSON形式でローカル保存、アプリ再起動後も参照可能）
- スケジュールの永続化（登録済みスケジュールをアプリ再起動後も保持）

### Changed
- 時刻選択を改善（時間：1時間単位、分：5分単位で選択）
- Git Worktreeブランチ表示を改善（利用可能な全ブランチを表示）

### Fixed
- Git Worktreeのブランチ名パース処理を修正
- 編集モード時のボタン表示を修正

## [1.0.0] - 2025-01-15

### Added
- 初回リリース
- CLIモードとGUIモードの両方をサポート
- スケジュール実行機能
- Git worktree対応
- Claude AI統合
- 実行履歴管理 