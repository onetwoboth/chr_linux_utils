# chr_linux_utils 🐧🦀

`chr_linux_utils` 是一个使用 **Rust** 编写的 Linux 基础命令工具集，  
目标是以 **安全、简洁、可维护** 的方式，重写常用的 Linux 命令行工具。

目前已实现（或计划实现）的命令包括：

- `chrls` —— Rust 版 `ls`
- `chrmkdir` —— Rust 版 `mkdir`
- `chrrm` —— Rust 版 `rm`

所有命令统一使用 `chr` 作为前缀，避免与系统自带命令冲突。

---

## 🚀 构建方式

使用 Cargo 构建 release 版本：

```bash
cargo build --release
```

构建完成后，可执行文件位于：

```rust
target/release/
```

例如：
- target/release/chrls
- target/release/chrmkdir

---

## 🛠️ 安装（本地测试）

将可执行文件复制到系统 PATH 目录中（如 /usr/bin）：

```bash
sudo cp target/release/chrls /usr/bin/
sudo cp target/release/chrmkdir /usr/bin/
```

然后即可直接使用：
```bash
chrls
chrmkdir test_dir
```

---

## 📦 RPM 打包
本项目提供了 RPM spec 文件，可用于在 **RHEL / Rocky Linux / AlmaLinux** 系统上构建和安装工具：
`chrls` 和 `chrmkdir`。

### 1. 安装构建依赖

```bash
sudo dnf install -y \
    rpm-build \
    rpmdevtools \
    rust \
    cargo \
    git
```

### 2. 准备 RPM 构建目录
初始化标准 RPM 构建目录：
```bash
rpmdevsetuptree
```

会在你的主目录下创建以下结构：
```text
~/rpmbuild/
├── BUILD
├── BUILDROOT
├── RPMS
├── SOURCES
├── SPECS
└── SRPMS
```

### 3. 创建源码 tarball
在项目根目录下执行：
```bash
git archive \
  --format=tar.gz \
  --prefix=chr_linux_utils-0.1.1/ \
  HEAD \
  -o chr_linux_utils-0.1.1.tar.gz
```

然后将生成的源码包移动到 RPM 的 SOURCES 目录：
```bash
mv chr_linux_utils-0.1.1.tar.gz ~/rpmbuild/SOURCES/
```

### 4. 安装 SPEC 文件
将 spec 文件复制到 RPM 的 SPECS 目录：
```bash
cp chr_linux_utils.spec ~/rpmbuild/SPECS/
```

### 5. 构建 RPM 包
```bash
rpmbuild -ba ~/rpmbuild/SPECS/chr_linux_utils.spec
```
构建成功后，生成的 RPM 包会位于：
```text
~/rpmbuild/RPMS/x86_64/
```

### 6. 安装 RPM 包
```bash
sudo dnf install ~/rpmbuild/RPMS/x86_64/chr_linux_utils-*.rpm
```
安装完成后，以下命令即可在系统中全局使用：
```bash
chrls
chrmkdir
chrrm
```

### 7. 注意事项
- 本项目默认禁用了调试信息包（debuginfo）。

- RPM 安装的二进制文件路径为 /usr/bin。

- 该打包流程遵循标准 RPM 最佳实践。

---

## 🧠 说明
- 本项目主要用于**学习和实践**