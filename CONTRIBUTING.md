# 貢献ガイド (Contributing Guide)

Claude Schedulerプロジェクトへの貢献を歓迎します！

## 開発環境のセットアップ

### 必要環境
- Rust 1.70以上
- Git
- Claude CLI（テスト用）

### 開発手順
```bash
# リポジトリをフォーク・クローン
git clone https://github.com/your-username/claude-scheduler.git
cd claude-scheduler

# 依存関係のインストール
cargo build

# 開発モードで実行
cargo run

# テスト実行
cargo test

# リリースビルド
cargo build --release
```

## 貢献方法

### 1. Issue報告
- バグ報告や機能提案は[Issues](https://github.com/your-username/claude-scheduler/issues)で
- 既存のIssueを確認してから新規作成してください
- 日本語・英語どちらでも大丈夫です

### 2. Pull Request
1. このリポジトリをフォーク
2. 機能ブランチを作成 (`git checkout -b feature/new-feature`)
3. 変更をコミット (`git commit -am 'Add new feature'`)
4. ブランチにプッシュ (`git push origin feature/new-feature`)
5. Pull Requestを作成

### 3. コードスタイル
- `cargo fmt` でコードフォーマット
- `cargo clippy` でLintチェック
- 警告なしでコンパイルできること

## プロジェクト構造

```
src/
├── main.rs              # エントリーポイント
├── models.rs            # データ構造定義
├── utils.rs             # 時間処理ユーティリティ
├── git.rs               # Git Worktree機能
└── components.rs        # UI コンポーネント
```

## 主要機能

- **Claude Code実行**: Claude AIとの連携
- **シェルコマンド実行**: 直接的なコマンド実行
- **Git Worktree並列実行**: 複数ブランチでの同時実行
- **スケジュール機能**: 時間指定による自動実行
- **実行履歴管理**: 全実行の記録と管理

## 改善提案

### 優先度高
- 永続化ストレージの実装
- エラーハンドリングの改善
- テストカバレッジの向上

### 優先度中
- 設定ファイル対応
- 通知機能
- 実行ログの改善

### 優先度低
- UI/UXの改善
- 多言語対応
- クロスプラットフォーム対応

## 質問・サポート

- GitHub Issues
- ディスカッション機能

## ライセンス

このプロジェクトはMITライセンスです。貢献コードもMITライセンスでライセンスされます。

---

ご協力ありがとうございます！🎉 