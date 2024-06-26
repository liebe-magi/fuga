[![Crates.io](https://img.shields.io/crates/v/fuga)](https://crates.io/crates/fuga)
<!-- ALL-CONTRIBUTORS-BADGE:START - Do not remove or modify this section -->
[![All Contributors](https://img.shields.io/badge/all_contributors-1-orange.svg?style=flat-square)](#contributors-)
<!-- ALL-CONTRIBUTORS-BADGE:END -->
[![Crates.io](https://img.shields.io/crates/l/fuga)](https://github.com/liebe-magi/fuga/blob/main/LICENSE)
[![build](https://github.com/liebe-magi/fuga/actions/workflows/build.yml/badge.svg?branch=main&event=push)](https://github.com/liebe-magi/fuga/actions/workflows/build.yml)

# 📦 FUGA 📦

![logo](/res/logo_256.jpg)

A CLI tool to operate files or directories in 2 steps.

[日本語のREADMEはこちら](README_jp.md)

## 📦 DESCRIPTION

- `fuga` is a CLI tool that performs file operations in two steps.
- Developed as an alternative to commands like `mv`, `cp`, and `ln`.
- Mark files or directories to operate on using `fuga mark`, and then perform copy or move operations after navigating to another directory.

## 📦 INSTALLATION

### Pre-built Binaries

- Pre-built binaries for the following architectures are available on [releases](https://github.com/liebe-magi/fuga/releases).

  - aarch64-apple-darwin (Mac - Apple Chip)
  - x86_64-apple-darwin (Mac - Intel Chip)
  - x86_64-unknown-linux-gnu (Linux - Intel Chip)

- Place the binary for your architecture in a directory included in your system's PATH.

### Build with Cargo

- You can install `fuga` by building it using the `cargo` command.

```
cargo install fuga
```

### Verify Installation

- If the installation is successful, the version information can be displayed using the following command:

```
$ fuga -V
fuga v0.1.1
```

## 📦 USAGE

```
A CLI tool to operate files or directories in 2 steps.

Usage: fuga <COMMAND>

Commands:
  mark        Set the path of the target file or directory
  copy        Copy the marked file or directory
  move        Move the marked file or directory
  link        Make a symbolic link to the marked file or directory
  completion  Generate the completion script
  version     Show the version of the tool
  help        Print this message or the help of the given subcommand(s)

Options:
  -h, --help     Print help
  -V, --version  Print version
```

### Setting the Target File

- Mark the file or directory you want to operate on with `fuga mark <TARGET>`.

```
$ fuga mark target_file.txt
✅ : 📄 target_file.txt has marked.
```

- To check the currently marked file or directory, use `fuga mark --show`.

```
$ fuga mark --show
ℹ️ : 📄 /home/user/path/to/file/target_file.txt
```

- To unmark a file or directory, use `fuga mark --reset`.

```
$ fuga mark --reset
✅ : The marked path has reset.
```

### File Operations

Three file operations are possible: `Copy`, `Move`, and `Symbolic Link creation`.

#### Copy

- Navigate to the destination directory and use `fuga copy` to copy the marked file or directory.

```
$ cd test_dir_copy

$ fuga copy
ℹ️ : Start copying 📄 target_file.txt from /home/user/path/to/file/target_file.txt
✅ : 📄 target_file.txt has copied.
```

- You can also specify the destination directory or file name.

```
$ fuga copy test_dir_copy
ℹ️ : Start copying 📄 test_dir_copy/target_file.txt from /home/user/path/to/file/target_file.txt
✅ : 📄 test_dir_copy/target_file.txt has copied.

$ fuga copy copy.txt
ℹ️ : Start copying 📄 copy.txt from /home/user/path/to/file/target_file.txt
✅ : 📄 copy.txt has copied.
```

#### Move

- Navigate to the destination directory and use `fuga move` to move the marked file or directory.

```
$ cd test_dir_move

$ fuga move
ℹ️ : Start moving 📄 target_file.txt from /home/user/path/to/file/target_file.txt
✅ : 📄 target_file.txt has moved.
```

- Similar to copying, you can specify the destination directory or file name.

```
$ fuga move test_dir_move
ℹ️ : Start copying 📄 test_dir_move/target_file.txt from /home/user/path/to/file/target_file.txt
✅ : 📄 test_dir_move/target_file.txt has moved.

$ fuga move move.txt
ℹ️ : Start moving 📄 move.txt from /home/user/path/to/file/target_file.txt
✅ : 📄 move.txt has moved.
```

#### Symbolic Link

- Navigate to the directory where you want to create a symbolic link and use `fuga link` to create a symbolic link to the marked file or directory.

```
$ cd test_dir_link

$ fuga link
ℹ️ : Start making symbolic link 📄 target_file.txt from /home/user/path/to/file/target_file.txt
✅ : 📄 target_file.txt has made.
```

- You can also specify the destination directory or file name for the symbolic link.

```
$ fuga link test_dir_link
ℹ️ : Start making symbolic link 📄 test_dir_link/target_file.txt from /home/user/path/to/file/target_file.txt
✅ : 📄 test_dir_link/target_file.txt has made.

$ fuga link link.txt
ℹ️ : Start making symbolic link 📄 link.txt from /home/user/path/to/file/target_file.txt
✅ : 📄 link.txt has made.
```

### Generating Completion Scripts

- Use `fuga completion <shell>` to output a script for command completion. It supports the following five shells:
  - bash
  - elvish
  - fish
  - powershell
  - zsh

```
# For fish
$ fuga completion fish > ~/.config/fish/completions/fuga.fish
```
## Contributors ✨

Thanks goes to these wonderful people ([emoji key](https://allcontributors.org/docs/en/emoji-key)):

<!-- ALL-CONTRIBUTORS-LIST:START - Do not remove or modify this section -->
<!-- prettier-ignore-start -->
<!-- markdownlint-disable -->
<table>
  <tbody>
    <tr>
      <td align="center" valign="top" width="14.28%"><a href="https://hackfront.dev"><img src="https://avatars.githubusercontent.com/u/38152917?v=4?s=100" width="100px;" alt="りーべ"/><br /><sub><b>りーべ</b></sub></a><br /><a href="#projectManagement-liebe-magi" title="Project Management">📆</a> <a href="https://github.com/liebe-magi/fuga/pulls?q=is%3Apr+reviewed-by%3Aliebe-magi" title="Reviewed Pull Requests">👀</a></td>
    </tr>
  </tbody>
</table>

<!-- markdownlint-restore -->
<!-- prettier-ignore-end -->

<!-- ALL-CONTRIBUTORS-LIST:END -->

This project follows the [all-contributors](https://github.com/all-contributors/all-contributors) specification. Contributions of any kind welcome!