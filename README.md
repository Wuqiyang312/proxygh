# proxygh

[![Rust](https://img.shields.io/badge/rust-1.0+-orange.svg)](https://www.rust-lang.org)
[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)

一个高性能、轻量级的 GitHub 资源反向代理服务，旨在解决网络访问受限问题。

首发于www.52pojie.cn

## ✨ 特性

-   **一站式代理**：无缝代理 GitHub 上的各类资源，包括文件源码、仓库归档（zip/tar.gz）以及 Release 资产。
-   **智能重定向跟随**：自动处理 GitHub 服务的各种 HTTP 重定向（301/302/307/308），确保最终请求能够到达真正的资源地址。
-   **SSRF 防护**：通过严格的域名白名单机制，仅允许代理请求发往 `github.com` 及其相关权威域名，确保服务器安全。
-   **高性能与异步**：基于 `tokio` 异步运行时和 `axum` 框架，具备出色的并发处理能力。
-   **简洁易用**：提供 Web 表单和 URL 路径两种代理方式。
-   **现代化 TLS**：使用 `rustls` 提供安全的 HTTPS 连接，支持 HTTP/1 和 HTTP/2。
-   **可观测性**：集成 `tracing` 日志，提供详细的请求记录。

## 🚀 快速开始

### 前置条件

-   安装 [Rust](https://www.rust-lang.org/tools/install)
-   安装 Microsoft C++ 生成工具 或者 GCC
-   安装 CMake
-   安装 NASM

### 1. 克隆项目

```bash
git clone https://github.com/Wuqiyang312/proxygh
cd proxygh
```

### 2. 运行服务

```bash
# 使用 cargo 直接运行
cargo run
```

服务将在 `http://0.0.0.0:3000` 上启动。

### 3. 使用代理

#### 方法一：通过 Web 界面

1.  在浏览器中打开 `http://localhost:3000`。
2.  在输入框中粘贴你想要下载的 GitHub 资源的完整 URL。
3.  点击 "Download" 按钮，浏览器将自动开始下载。

**例如**：输入 `https://github.com/torvalds/linux/archive/refs/heads/master.zip` 即可开始下载 Linux 内核源码包。

#### 方法二：通过 URL 路径

将 GitHub 的 URL 直接附加到 `http://localhost:3000/gh/` 之后。

**格式**：`http://localhost:3000/gh/<GITHUB_URL>`

**示例**：

-   代理 `raw` 文件：
    `http://localhost:3000/gh/raw.githubusercontent.com/user/repo/main/README.md`

-   代理 Release 资产：
    `http://localhost:3000/gh/github.com/user/repo/releases/download/v1.0.0/app.exe`

-   代理仓库源码包：
    `http://localhost:3000/gh/codeload.github.com/user/repo/zip/refs/heads/main`

## 📦 构建生产版本

使用 `--release` 标志进行优化编译，以获得最佳性能。

```bash
cargo build --release
```

编译后的二进制文件将位于 `target/release/proxygh`。

## ⚙️ 配置

当前版本的配置（如监听端口、日志级别）是硬编码在 `main.rs` 中的。未来版本可能会支持通过配置文件或环境变量进行配置。

-   **监听地址**：`0.0.0.0:3000`
-   **最大重定向次数**：`10`
-   **允许的域名白名单**：
    -   `github.com`
    -   `raw.githubusercontent.com`
    -   `codeload.github.com`
    -   `release-assets.githubusercontent.com`

## 🛠️ 技术栈

-   **Web 框架**: [axum](https://github.com/tokio-rs/axum)
-   **异步运行时**: [tokio](https://tokio.rs/)
-   **HTTP 客户端**: [hyper](https://github.com/hyperium/hyper)
-   **TLS 后端**: [rustls](https://github.com/rustls/rustls)
-   **日志**: [tracing](https://github.com/tokio-rs/tracing)
-   **URL 解析**: [url](https://github.com/servo/rust-url)

## 🤝 贡献

欢迎提交 Issue 和 Pull Request！

1.  Fork 本仓库。
2.  创建你的特性分支 (`git checkout -b feature/AmazingFeature`)。
3.  提交你的更改 (`git commit -m 'Add some AmazingFeature'`)。
4.  推送到分支 (`git push origin feature/AmazingFeature`)。
5.  打开一个 Pull Request。

## 📄 许可证

本项目采用 MIT 许可证。详情请参阅 [LICENSE](LICENSE) 文件。
