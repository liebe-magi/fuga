[![Crates.io](https://img.shields.io/crates/v/fugue-box)](https://crates.io/crates/fugue-box)
[![Crates.io](https://img.shields.io/crates/l/fugue-box)](https://github.com/MagicalLiebe/fugue/blob/main/LICENSE)
[![CI](https://github.com/MagicalLiebe/fugue/actions/workflows/rust_ci.yml/badge.svg?branch=develop)](https://github.com/MagicalLiebe/fugue/actions/workflows/rust_ci.yml)

# 📦 FUGUE 📦

A CLI tool to operate files or directories in 2 steps.

## 📦 DESCRIPTION

- `fugue`はファイル操作を2ステップで行うCLIツールです。
- `mv`,`cp`,`ln`コマンドなどの代替コマンドとして開発しました。
- 操作対象のファイルやディレクトリを`fugue mark`によりマーキングし、別のディレクトリに移動した後にコピーや移動を実行できます。

## 📦 INSTALLATION

### ビルド済みバイナリ

- 以下のアーキテクチャ用のバイナリを[releases](https://github.com/MagicalLiebe/fugue/releases)に準備しています。

  - aarch64-apple-darwin (Mac - Apple Chip)
  - x86_64-apple-darwin (Mac - Intel Chip)
  - x86_64-unknown-linux-gnu (Linux - Intel Chip)

- お使いのPCにあったバイナリをパスの通ったディレクトリに配置してください。

### Cargoによるビルド

- `cargo`コマンドによりビルドすることでインストールできます。

```
$ cargo install fugue-box
```

### コマンドの確認

- 以下のコマンドでバージョン情報が表示されればインストール完了です。

```
$ fugue -V
fugue v0.0.3
```

## 📦 USAGE

```
USAGE:
    fugue <SUBCOMMAND>

OPTIONS:
    -h, --help       Print help information
    -V, --version    Print version information

SUBCOMMANDS:
    copy       Copy the marked file or directory
    help       Print this message or the help of the given subcommand(s)
    link       Make a symbolic link to the marked file or directory
    mark       Set the path of the target file or directory
    move       Move the marked file or directory
    version    Show the version of the tool
```

### 操作対象ファイルの設定

- `fugue mark <TARGET>`で操作対象とするファイルやディレクトリをマーキングします。

```
$ fugue mark target_file.txt
✅ : 📄 target_file.txt has marked.
```

- 現在マーキング中のファイルやディレクトリを確認したいときは、`fugue mark --show`で確認できます。

```
$ fugue mark --show
ℹ️ : 📄 /home/user/path/to/file/target_file.txt
```

- マーキングを解除したい場合は、`fugue mark --reset`で解除できます。

```
$ fugue mark --reset
✅ : The marked path has reset.
```

### ファイル操作

以下の3つのファイル操作が可能です。

#### コピー

- コピー先のディレクトリに移動し、`fugue copy`でマーキング中のファイルやディレクトリをコピーできます。

```
$ cd test_dir_copy

$ fugue copy
ℹ️ : Start copying 📄 target_file.txt from /home/user/path/to/file/target_file.txt
✅ : 📄 target_file.txt has copied.
```

- コピー先のディレクトリやファイル名を与えることも可能です。

```
$ fugue copy test_dir_copy
ℹ️ : Start copying 📄 test_dir_copy/target_file.txt from /home/user/path/to/file/target_file.txt
✅ : 📄 test_dir_copy/target_file.txt has copied.

$ fugue copy copy.txt
ℹ️ : Start copying 📄 copy.txt from /home/user/path/to/file/target_file.txt
✅ : 📄 copy.txt has copied.
```

#### 移動

- 移動先のディレクトリに移動し、`fugue move`でマーキング中のファイルやディレクトリを移動できます。

```
$ cd test_dir_move

$ fugue move
ℹ️ : Start moving 📄 target_file.txt from /home/user/path/to/file/target_file.txt
✅ : 📄 target_file.txt has moved.
```

- コピー同様、移動先のディレクトリやファイル名を与えることも可能です。

```
$ fugue move test_dir_move
ℹ️ : Start copying 📄 test_dir_move/target_file.txt from /home/user/path/to/file/target_file.txt
✅ : 📄 test_dir_move/target_file.txt has moved.

$ fugue move move.txt
ℹ️ : Start moving 📄 move.txt from /home/user/path/to/file/target_file.txt
✅ : 📄 move.txt has moved.
```

#### シンボリックリンク

- シンボリックリンクを作成したいディレクトリに移動し、`fugue link`でマーキング中のファイルやディレクトリへのシンボリックリンクを作成できます。

```
$ cd test_dir_link

$ fugue link
ℹ️ : Start making symbolic link 📄 target_file.txt from /home/user/path/to/file/target_file.txt
✅ : 📄 target_file.txt has made.
```

- シンボリックリンク作成先のディレクトリやファイル名を与えることも可能です。

```
$ fugue link test_dir_link
ℹ️ : Start making symbolic link 📄 test_dir_link/target_file.txt from /home/user/path/to/file/target_file.txt
✅ : 📄 test_dir_link/target_file.txt has made.

$ fugue link link.txt
ℹ️ : Start making symbolic link 📄 link.txt from /home/user/path/to/file/target_file.txt
✅ : 📄 link.txt has made.
```
