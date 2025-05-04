# For DEVELOPERS

## Development Workflow

This project follows the **GitHub Flow** model.

1. Work on the latest `main` branch.
2. Create a new **feature branch** from `main`:
   ```sh
   git checkout -b feature/<short-description>
   ```
3. Push your changes and open a pull request (PR).

4. Wait for GitHub Actions to pass and request review if needed.

5. Once approved, merge into main.

> ✅ Do not commit directly to main.

## Project Structure

```
cmdplus/
├── src/
│   ├── commands/        # One module per subcommand
│   │   ├── touch.rs
│   │   ├── which.rs
│   │   └── lsz.rs
│   ├── cli.rs           # CLI parser with clap
│   └── main.rs          # Entry point
│   └── lib.rs
│   └── commands.rs
├── tests/               # Integration tests
├── .github/
│   └── workflows/ci.yml # GitHub Actions config
├── Cargo.toml
└── README.md
└── DEVELOP.md
└── LICENSE
```

## Naming Conventions

- Filenames: snake_case (e.g., touch.rs)
- Public functions: <command>_command (e.g., touch_command)
- Test files: <command>_test.rs
- Commit massages:
    - feat: A new feature
    - fix: A bug fix
    - docs: Documentation only changes
    - style: Changes that do not affect the meaning of the code (white-space, formatting, missing semi-colons, etc)
    - refactor: A code change that neither fixes a bug nor adds a feature
    - perf: A code change that improves performance
    - test: Adding missing or correcting existing tests
    - chore: Changes to the build process or auxiliary tools and libraries such as documentation generation
- Otherwise, follow the Rust naming conventions.

## Coding Standards

- Run `cargo fmt` before pushing
- Ensure `cargo clippy` and cargo test pass locally

## GitHub Actions

CI runs on every PR and push to main. It checks:

- Formatting: `cargo fmt --check`

- Linting: `cargo clippy`

- Testing: `cargo test`

## Contributing

### We welcome contributions!

How to contribute:

1. Fork the repository

2. Follow the development workflow above

3. Keep PRs focused and descriptive

4. Add tests where appropriate

5. Follow the established code style

---

## 開発フロー

このプロジェクトは GitHub Flow に従って運用されています。

1. main ブランチが常に最新の開発ベースです。

2. 機能追加などは新しい feature ブランチ から始めます：
    ```sh
    git checkout -b feature/<短い説明>
    ```
3. 作業が終わったら GitHub に push して プルリクエスト (PR) を作成します。

4. GitHub Actions のチェックを通過し、必要ならレビューを依頼します。

5. レビュー後、main にマージします。

> ✅ main への直接コミットは禁止です。

## プロジェクト構成

```
cmdplus/
├── src/
│   ├── commands/        # サブコマンドごとのモジュール
│   │   ├── touch.rs
│   │   ├── which.rs
│   │   └── lsz.rs
│   ├── cli.rs           # clap による CLI パーサ
│   └── main.rs          # エントリーポイント
│   └── lib.rs
│   └── commands.rs
├── tests/               # テスト
├── .github/
│   └── workflows/ci.yml # GitHub Actions の設定
├── Cargo.toml
└── README.md
└── DEVELOP.md
└── LICENSE
```

## 命名規則

- ファイル名: スネークケース（例：`touch.rs`）

- 公開関数: <コマンド名>_command（例：`touch_command`）

- テストファイル: `<コマンド>_test.rs`
    - コミットメッセージ:
        - feat: 新しい機能
        - fix: バグの修正
        - docs: ドキュメントのみの変更
        - style: 空白、フォーマット、セミコロン追加など
        - refactor: 仕様に影響がないコード改善(リファクタ)
        - perf: パフォーマンス向上関連
        - test: テスト関連
        - chore: ビルド、補助ツール、ライブラリ関連
- その他Rustコーディングスタイルに従います。

## GitHub Actions について

PR または main への push ごとに CI が以下の通り自動実行されます。：

- フォーマット: `cargo fmt --check`

- 静的解析: `cargo clippy`

- テスト: `cargo test`

## コントリビュートについて

### コントリビュートは大歓迎です！

本プロジェクトの利便性向上、バグフィックスにぜひご協力ください。

参加方法:

1. リポジトリをフォークしてください

2. 上記の開発フローに従ってください

3. PR は小さく、分かりやすくしてください

4. 必要に応じてテストを追加してください

5. 既存のコードスタイルに従ってください