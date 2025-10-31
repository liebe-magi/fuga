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
- よく使うマークリストはプリセットとして保存し、CLIやダッシュボードからいつでも再読み込みできます。
- 引数なしで`fuga`を起動すると対話的なダッシュボードTUIが立ち上がり、ディレクトリ移動やマーキング、コピー/移動/リンク操作をターミナル内で完結できます。

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
fuga v1.1.0
```

## 📦 USAGE

> サブコマンドなしで`fuga`を実行するとダッシュボードTUIが起動します。バッチ処理やスクリプト用途では以下のサブコマンドを利用してください。

```
A CLI tool to operate files or directories in 2 steps.

Usage: fuga <COMMAND>

Commands:
  mark        Manage the marked targets
  copy        Copy the marked targets
  move        Move the marked targets
  link        Make symbolic links to the marked targets
  completion  Generate the completion script
  preset      Manage mark presets
  version     Show the version of the tool
  help        Print this message or the help of the given subcommand(s)

Options:
  -h, --help     Print help
  -V, --version  Print version
```

### インタラクティブダッシュボード (TUI)

- 引数なしで`fuga`を起動するとカレントディレクトリをブラウズするダッシュボードが表示されます。
- `.`や`Ctrl+h`で隠しファイルの表示を切り替え、`/`を押してファジー検索で絞り込みできます。
- カーソル移動は矢印キーや`j`/`k`、ディレクトリの開閉は`Enter`/`l`、親ディレクトリへ戻るには`h`または`Backspace`を利用できます。
- `m`またはスペースでマークのオン/オフ、`Ctrl+r`または`R`でマーク一覧をリセット、`?`で操作方法のヘルプを確認できます。
- `P`でプリセット読み込みポップアップを開き、`S`で現在のマークをプリセットとして保存、ポップアップ内で`D`または`x`を押すとハイライト中のプリセットを削除できます。
- `c`/`v`/`s`でそれぞれコピー/移動/シンボリックリンクを現在ブラウズ中のディレクトリに対して実行し、`q`で変更なしに終了します。

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

### プリセット管理

- 現在のマーク一覧をプリセットとして保存するには`fuga preset save <NAME>`を使用します。

```
$ fuga preset save photos
✅ : Preset 'photos' saved with 3 target(s).
```

- 保存したプリセットをマーク一覧に再読み込みするには`fuga preset load <NAME>`を使用します。

```
$ fuga preset load photos
✅ : Preset 'photos' loaded. Mark list now tracks 3 target(s).
```

- 登録済みのプリセット一覧は`fuga preset list`で確認でき、`fuga preset show <NAME>`で内容を表示、`fuga preset delete <NAME>`で不要なプリセットを削除できます。

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