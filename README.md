# 📦 FUGUE 📦

A CLI tool to operate files or directories in 2 steps.

## 📦 DESCRIPTION

- `fugue`はファイル操作を2ステップで行うCLIツールです。
- `mv`,`cp`,`ln`コマンドなどの代替コマンドとして開発しました。
- 操作対象のファイルやディレクトリを`fugue mark`によりマーキングし、別のディレクトリに移動した後にコピーや移動を実行できます。

## 📦 INSTALLATION

- `cargo`コマンドによりビルドすることでインストールできます。
  - ビルド済みのバイナリファイルは今後準備する予定です。

```
$ cargo install fugue-box
```

以下のコマンドでバージョン情報が表示されればインストール完了です。

```
$ fugue -V
fugue v0.0.1
```

## 📦 USAGE

```
USAGE:
    fugue <SUBCOMMAND>

OPTIONS:
    -h, --help       Print help information
    -V, --version    Print version information

SUBCOMMANDS:
    copy       Copy the marked files or directories
    help       Print this message or the help of the given subcommand(s)
    link       Make a symbolic link of the marked files or directories
    mark       Set the path of the target file or directory
    move       Move the marked files or directories
    version    Show the version of the tool
```
