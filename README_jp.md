[![Crates.io](https://img.shields.io/crates/v/fuga)](https://crates.io/crates/fuga)
[![Crates.io](https://img.shields.io/crates/l/fuga)](https://github.com/liebe-magi/fuga/blob/main/LICENSE)
[![build](https://github.com/liebe-magi/fuga/actions/workflows/build.yml/badge.svg?branch=main&event=push)](https://github.com/liebe-magi/fuga/actions/workflows/build.yml)

# 📦 FUGA 📦

![logo](/res/logo_256.jpg)

A CLI tool to operate files or directories in 2 steps.

## 📦 DESCRIPTION

- `fuga`はファイル操作を2ステップで行うCLIツールです。
- `mv`,`cp`,`ln`コマンドなどの代替コマンドとして開発しました。
- `fuga mark`で操作対象のファイルやディレクトリを複数マーキングし、別ディレクトリに移動してからまとめてコピー/移動/リンクできます。

## 📦 INSTALLATION

### ビルド済みバイナリ

- 以下のアーキテクチャ用のバイナリを[releases](https://github.com/liebe-magi/fuga/releases)に準備しています。

  - aarch64-apple-darwin (Mac - Apple Chip)
  - x86_64-apple-darwin (Mac - Intel Chip)
  - x86_64-unknown-linux-gnu (Linux - Intel Chip)

- お使いのPCにあったバイナリをパスの通ったディレクトリに配置してください。

### Cargoによるビルド

- `cargo`コマンドによりビルドすることでインストールできます。

```
cargo install fuga
```

### コマンドの確認

- 以下のコマンドでバージョン情報が表示されればインストール完了です。

```
$ fuga -V
fuga v0.1.1
```

## 📦 USAGE

```
A CLI tool to operate files or directories in 2 steps.

Usage: fuga <COMMAND>

Commands:
  mark        Manage the marked targets
  copy        Copy the marked targets
  move        Move the marked targets
  link        Make symbolic links to the marked targets
  completion  Generate the completion script
  version     Show the version of the tool
  help        Print this message or the help of the given subcommand(s)

Options:
  -h, --help     Print help
  -V, --version  Print version
```

### マーク対象の管理

- `fuga mark <PATH...>`で操作対象とするファイルやディレクトリを一括マーキングします。

```
$ fuga mark target_file.txt docs
✅ : 📄 /home/user/path/to/target_file.txt marked.
✅ : 📁 /home/user/path/to/docs marked.
ℹ️  : Mark list now tracks 2 target(s).
```

- 既存のマークに重複なく追加したいときは、`fuga mark --add <PATH...>`を利用します。

```
$ fuga mark --add images/*.png
✅ : 📄 /home/user/path/to/images/banner.png added.
✅ : 📄 /home/user/path/to/images/logo.png added.
ℹ️  : Mark list now tracks 4 target(s).
```

- 現在マーキング中のターゲットを確認したい場合は、`fuga mark --list`で一覧表示できます。

```
$ fuga mark --list
ℹ️  : Marked targets:
📄 /home/user/path/to/target_file.txt
📁 /home/user/path/to/docs
📄 /home/user/path/to/images/banner.png
📄 /home/user/path/to/images/logo.png
```

- マーキングを全て解除したい場合は、`fuga mark --reset`を利用します。

```
$ fuga mark --reset
✅ : Marked targets cleared.
ℹ️  : Mark list now tracks 0 target(s).
```

### ファイル操作

以下の3つのファイル操作が可能です。

#### コピー

- コピー先のディレクトリに移動し、`fuga copy`でマーキング中のファイルやディレクトリをコピーできます。

```
$ cd test_dir_copy

$ fuga copy
ℹ️  : Copying 📄 /home/user/path/to/target_file.txt -> /current/dir/target_file.txt
✅ : 📄 /current/dir/target_file.txt copied.
ℹ️  : Copying 📁 /home/user/path/to/docs -> /current/dir/docs
✅ : 📁 /current/dir/docs copied。
```

- コピー先のディレクトリやファイル名を与えることも可能です。

```
$ fuga copy test_dir_copy
ℹ️  : Copying 📄 /home/user/path/to/target_file.txt -> test_dir_copy/target_file.txt
✅ : 📄 test_dir_copy/target_file.txt copied.

$ fuga copy copy.txt
ℹ️  : Copying 📄 /home/user/path/to/target_file.txt -> copy.txt
✅ : 📄 copy.txt copied.
```

#### 移動

- 移動先のディレクトリに移動し、`fuga move`でマーキング中のファイルやディレクトリを移動できます。

```
$ cd test_dir_move

$ fuga move
ℹ️  : Moving 📄 /home/user/path/to/target_file.txt -> /current/dir/target_file.txt
✅ : 📄 /current/dir/target_file.txt moved.
ℹ️  : Moving 📁 /home/user/path/to/docs -> /current/dir/docs
✅ : 📁 /current/dir/docs moved。
ℹ️  : Mark list cleared after move.
```

- コピー同様、移動先のディレクトリやファイル名を与えることも可能です。

```
$ fuga move test_dir_move
ℹ️  : Moving 📄 /home/user/path/to/target_file.txt -> test_dir_move/target_file.txt
✅ : 📄 test_dir_move/target_file.txt moved.

$ fuga move move.txt
ℹ️  : Moving 📄 /home/user/path/to/target_file.txt -> move.txt
✅ : 📄 move.txt moved.
```

#### シンボリックリンク

- シンボリックリンクを作成したいディレクトリに移動し、`fuga link`でマーキング中のファイルやディレクトリへのシンボリックリンクを作成できます。

```
$ cd test_dir_link

$ fuga link
ℹ️  : Linking 📄 /home/user/path/to/target_file.txt -> /current/dir/target_file.txt
✅ : 📄 /current/dir/target_file.txt linked.
```

- シンボリックリンク作成先のディレクトリやファイル名を与えることも可能です。

```
$ fuga link test_dir_link
ℹ️  : Linking 📄 /home/user/path/to/target_file.txt -> test_dir_link/target_file.txt
✅ : 📄 test_dir_link/target_file.txt linked.

$ fuga link link.txt
ℹ️  : Linking 📄 /home/user/path/to/target_file.txt -> link.txt
✅ : 📄 link.txt linked.
```

### 補完スクリプトの生成

- `fuga completion <shell>`でコマンドの補完用スクリプトを標準出力します。シェルは以下の5つに対応しています。
  - bash
  - elvish
  - fish
  - powershell
  - zsh

```
# fishの場合
$ fuga completion fish > ~/.config/fish/completions/fuga.fish
```