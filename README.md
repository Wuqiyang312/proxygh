# proxygh

[![Rust](https://img.shields.io/badge/rust-1.0+-orange.svg)](https://www.rust-lang.org)
[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)

一个高性能、轻量级的 GitHub 资源反向代理服务，旨在解决网络访问受限问题。

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

### 使用 Docker (推荐)

项目包含 Dockerfile，可以直接使用 Docker 部署。Dockerfile 使用多阶段构建和 musl 静态编译，生成轻量级的 Alpine 镜像：

#### 构建镜像

```bash
docker build -t proxygh .
```

#### 运行容器

```bash
# 使用默认端口 3000
docker run -p 3000:3000 proxygh

# 自定义端口（例如 9090）
docker run -p 9090:9090 -e PORT=9090 proxygh
```

#### 使用 Docker Compose

如果项目中有 docker-compose.yml：

```bash
docker-compose up -d
```

#### Docker 构建细节

- **多阶段构建**：使用 `Rust:1.92.0-bullseye` 进行编译，最终镜像基于 `Alpine:3.23`，大幅降低镜像体积
- **静态编译**：使用 `x86_64-unknown-linux-musl` 目标进行交叉编译，生成静态链接二进制，兼容所有 Linux 环境
- **依赖优化**：只在最终镜像中包含必要的 CA 证书，保持镜像轻量级

### 本地开发环境配置

如果选择本地开发和构建，请确保满足以下前置条件：

#### Windows
-   安装 [Rust](https://www.rust-lang.org/tools/install)
-   安装 Microsoft C++ 生成工具（Visual Studio Build Tools 或 Visual Studio Community）
-   安装 [CMake](https://cmake.org/download/)
-   安装 [NASM](https://www.nasm.us/)

#### Linux (Ubuntu/Debian)
```bash
# 安装 Rust
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

# 安装依赖
sudo apt-get update
sudo apt-get install -y build-essential cmake nasm pkg-config
```

#### macOS
```bash
# 安装 Rust
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

# 使用 Homebrew 安装依赖
brew install cmake nasm
```

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

## 📦 构建与发布

### 本地构建生产版本

使用 `--release` 标志进行优化编译，以获得最佳性能：

```bash
cargo build --release
```

编译后的二进制文件将位于 `target/release/proxygh`。

### 构建优化配置

项目在 `Cargo.toml` 中配置了生产版本的优化选项：

```toml
[profile.release]
pie = true          # 位置无关可执行文件 (PIE)
strip = true        # 移除符号表
lto = true          # 链接时优化 (LTO)
```

这些设置确保最终二进制文件体积最小且性能最优。

### 静态编译

项目支持生成完全静态链接的二进制文件，不依赖任何系统库。这在 Docker 或跨平台部署时非常有用。

使用 `x86_64-unknown-linux-musl` 目标进行交叉编译：

```bash
# 添加 musl 目标
rustup target add x86_64-unknown-linux-musl

# 编译静态二进制
cargo build --release --target x86_64-unknown-linux-musl
```

生成的二进制文件位于 `target/x86_64-unknown-linux-musl/release/proxygh`。

## ⚙️ 配置

### 环境变量配置

项目通过 `.env` 文件加载环境变量，支持以下配置：

| 变量名 | 说明 | 默认值 |
|--------|------|--------|
| `PORT` | 应用监听端口 | `3000` |

#### 使用示例

创建 `.env` 文件：

```env
# 应用监听端口
PORT=9090
```

然后运行应用：

```bash
cargo run
# 或者
./proxygh  # 使用编译后的二进制
```

应用将在 `http://0.0.0.0:9090` 上启动。

### 硬编码配置

以下配置项当前硬编码在代码中：

-   **监听地址**：`0.0.0.0`
-   **最大重定向次数**：`10`
-   **允许的域名白名单**：
    -   `github.com`
    -   `raw.githubusercontent.com`
    -   `codeload.github.com`
    -   `release-assets.githubusercontent.com`

## 🛠️ 技术栈

-   **Web 框架**: [axum](https://github.com/tokio-rs/axum) - 高性能异步 Web 框架
-   **异步运行时**: [tokio](https://tokio.rs/) - 异步任务执行
-   **HTTP 客户端**: [hyper](https://github.com/hyperium/hyper) - 高性能 HTTP 客户端
-   **TLS 后端**: [rustls](https://github.com/rustls/rustls) - 现代化 TLS 实现
-   **日志**: [tracing](https://github.com/tokio-rs/tracing) - 结构化日志框架
-   **URL 解析**: [url](https://github.com/servo/rust-url) - URL 解析和操作
-   **环境变量**: [dotenv](https://github.com/dotenv-rs/dotenv) - `.env` 文件加载

## 📚 快速参考

### 常见命令

```bash
# 开发模式运行
cargo run

# 构建生产版本
cargo build --release

# 构建 Docker 镜像
docker build -t proxygh .

# 运行 Docker 容器（使用默认端口）
docker run -p 3000:3000 proxygh

# 运行 Docker 容器（自定义端口）
docker run -p 9090:9090 -e PORT=9090 proxygh

# 静态编译（Linux）
cargo build --release --target x86_64-unknown-linux-musl
```

### 访问示例

#### Web 界面
- 默认端口：`http://localhost:3000`
- 自定义端口：`http://localhost:9090`（如果设置了 `PORT=9090`）

#### 直接代理
- 代理 GitHub 仓库：`http://localhost:3000/gh/github.com/user/repo/archive/main.zip`
- 代理 Raw 文件：`http://localhost:3000/gh/raw.githubusercontent.com/user/repo/main/README.md`

## 🐛 故障排查

### 问题：Docker 构建失败

**症状**：`cargo build` 在 Docker 中报错

**解决方案**：
- 确保 Dockerfile 中的依赖都已安装
- 检查是否有足够的磁盘空间
- 清除 Docker 缓存：`docker build --no-cache -t proxygh .`

### 问题：无法访问 GitHub 资源

**症状**：代理请求返回 403 Forbidden

**解决方案**：
- 确保目标 URL 的域名在白名单中
- 检查网络连接是否正常
- 查看日志输出，确认请求是否被正确转发

### 问题：端口已被占用

**症状**：启动应用时报 "Address already in use" 错误

**解决方案**：
- 使用 `.env` 文件修改监听端口：`PORT=9090`
- 或者杀死占用该端口的进程

### 问题：HTTPS 连接失败

**症状**：代理 HTTPS 资源时出错

**解决方案**：
- 确保 Docker 镜像中包含 CA 证书（已在 Dockerfile 中配置）
- 本地构建时，确保系统的 TLS 配置正确

## 📖 其他资源

- [Rust 官方文档](https://www.rust-lang.org/)
- [Axum 框架文档](https://docs.rs/axum/)
- [Tokio 异步运行时](https://tokio.rs/)
- [GitHub API 文档](https://docs.github.com/en/rest)

## 🤝 贡献

欢迎提交 Issue 和 Pull Request！

1.  Fork 本仓库。
2.  创建你的特性分支 (`git checkout -b feature/AmazingFeature`)。
3.  提交你的更改 (`git commit -m 'Add some AmazingFeature'`)。
4.  推送到分支 (`git push origin feature/AmazingFeature`)。
5.  打开一个 Pull Request。

## 📄 许可证

本项目采用 MIT 许可证。详情请参阅 [LICENSE](LICENSE) 文件。
