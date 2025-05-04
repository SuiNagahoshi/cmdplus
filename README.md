# cmdplus

![build](https://github.com/your-username/cmdplus/actions/workflows/ci.yml/badge.svg)
[![License](https://img.shields.io/badge/license-Apache--2.0-blue.svg)](LICENSE)
![Rust](https://img.shields.io/badge/rust-stable-blue)

> Cross-platform CLI tools for Windows/macOS/Linux inspired by UNIX shell commands.

---

## Overview

`cmdplus` is a simple CLI toolset that brings essential UNIX-like commands to platforms where they’re missing, such as
Windows.  
It aims to replicate the behavior of frequently used Linux commands, while also adding useful original utilities.

## Features

- ✅ `touch` — Create a file and parent directories if needed.
- ✅ `which` — Locate a command in the system's PATH.
- ✅ `lsz` — List files/directories sorted by size.
- ✅ `rm` — Remove a file or directories.
- 🔜 `mktree` — Create complex directory structures.
- 🔜 `clipout` — Copy stdout to clipboard.
- 🔜 `filehead` — Show the first line of multiple files.

## Installation

### Install via [cargo](https://doc.rust-lang.org/cargo/):

Requires [Rust](https://www.rust-lang.org/tools/install).

```sh
cargo install --path .
```

### Download Prebuilt Binary

Visit the Releases page and download the appropriate binary for your platform:

- cmdplus-x86_64-pc-windows-msvc.exe (Windows)

- 🔜cmdplus-x86_64-unknown-linux-gnu (Linux)

- 🔜cmdplus-x86_64-apple-darwin (macOS)

After downloading:

```sh
# (Optional) Move to a directory in PATH
mv cmdplus-* /usr/local/bin/cmdplus
chmod +x /usr/local/bin/cmdplus
```

## Usage

```sh
cmdplus touch example.txt
cmdplus which cmd
cmdplus lsz ./
```

## Supported Platforms

- ✅ Windows
- 🔜Linux
- 🔜macOS

## Development

See [DEVELOP.md](./DEVELOP.md), [📚 Docs](https://suinagahoshi.github.io/cmdplus/) for contribution guidelines, branch
management, and development workflow (GitHub Flow).

## License

Licensed under the Apache License 2.0.

Read the [License](./LICENSE) file.

---
Made with ❤️ using Rust.


---

## 概要

cmdplus は、Linux や macOS で一般的に使われている便利なCLIコマンドを、Windowsなどのプラットフォームにも提供するクロスプラットフォーム対応ツール群です。
Linuxでおなじみのコマンドに加えて、オリジナルの便利ツールも実装しています。

## コマンド

- ✅ touch — ファイルを作成（必要ならディレクトリも作成）
- ✅ which — コマンドのフルパスを表示
- ✅ lsz — ファイル／フォルダをサイズ順に表示
- ✅ `rm` — ファイルやディレクトリを削除

- 🔜 mktree — 複雑なフォルダ構造を一括作成
- 🔜 clipout — 標準出力をクリップボードへコピー
- 🔜 filehead — 複数ファイルの先頭行をまとめて表示

## インストール

### Cargoからインストール

本リポジトリをクローンまたはダウンロードし、Rustがインストールされている環境で以下を実行

```sh
cargo install --path .
```

### バイナリをダウンロード

Releasesページ から、自分のOSに合ったバイナリをダウンロードできます。

必要に応じて実行権限を付与してください。

```sh
# (Optional) Move to a directory in PATH
mv cmdplus-* /usr/local/bin/cmdplus
chmod +x /usr/local/bin/cmdplus
```

### 利用方法

```sh
cmdplus touch ファイル名.txt
cmdplus which コマンド名
cmdplus lsz ./
```

## 対応プラットフォーム

- ✅ Windows
- 🔜Linux
- 🔜macOS

## 開発について

開発フローやコントリビューションに関する情報は [DEVELOP.md](./DEVELOP.md), [📚 Docs](https://suinagahoshi.github.io/cmdplus/)
をご覧ください。

## ライセンス

本リポジトリはApache License 2.0 のもとで公開されています。
詳細は [LICENSE](./LICENSE) をご確認ください。