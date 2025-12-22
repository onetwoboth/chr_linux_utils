# chr_linux_utils 🐧🦀

`chr_linux_utils` 是一个使用 **Rust** 编写的 Linux 基础命令工具集，  
目标是以 **安全、简洁、可维护** 的方式，重写常用的 Linux 命令行工具。

目前已实现（或计划实现）的命令包括：

- `chrls` —— Rust 版 `ls`
- `chrmkdir` —— Rust 版 `mkdir`
- （持续扩展中……）

所有命令统一使用 `chr` 作为前缀，避免与系统自带命令冲突。

---

## ✨ 项目目标

- 使用 **Rust 标准库** 实现核心功能  
- 学习并实践：
  - Rust 的 binary crate / lib crate 结构
  - Linux 文件系统与权限模型
  - 命令行工具设计
  - RPM 打包流程（`.spec` 文件）
- 为后续更深入的系统工具开发打基础

---

## 📦 项目结构

```text
chr_linux_utils/
├── Cargo.toml
└── src
    ├── lib.rs        # 公共逻辑（参数解析、工具函数等）
    └── bin
        ├── chrls.rs  # chrls 命令入口
        └── chrmkdir.rs # chrmkdir 命令入口

---

## 🚀 构建方式

使用 Cargo 构建 release 版本：

```bash
cargo build --release

构建完成后，可执行文件位于：

```rust
target/release/

例如：
- target/release/chrls
- target/release/chrmkdir

## 🛠️ 安装（本地测试）

将可执行文件复制到系统 PATH 目录中（如 /usr/bin）：

```bash
sudo cp target/release/chrls /usr/bin/
sudo cp target/release/chrmkdir /usr/bin/

然后即可直接使用：
```bash
chrls
chrmkdir test_dir

## 📦 RPM 打包（计划 / 进行中）
本项目计划支持：

- 打包为 tar.gz

- 编写 .spec 文件

- 生成并安装 .rpm 包

- 在 Red Hat / Rocky / CentOS 系列系统上使用

## 🧠 说明
- 本项目主要用于**学习和实践**