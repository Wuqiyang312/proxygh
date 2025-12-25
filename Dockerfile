# 使用官方 Rust 镜像作为构建环境
FROM rust:1.92.0-bullseye AS builder

# 安装 musl 工具链和构建依赖
RUN apt-get update && apt-get install -y \
    cmake nasm build-essential pkg-config musl-tools

WORKDIR /app

# 复制 Cargo 文件
COPY Cargo.toml Cargo.lock ./

# 创建虚拟 main.rs 以解析依赖（避免 cargo build 失败）
RUN mkdir src && echo 'fn main() {}' > src/main.rs

# 复制真实源码
COPY src ./src

# 使用 musl 目标交叉编译，生成静态链接二进制
RUN rustup target add x86_64-unknown-linux-musl && \
    cargo build --release --locked --target x86_64-unknown-linux-musl

# 运行阶段：使用 Alpine
FROM alpine:3.23

RUN apk add --no-cache ca-certificates

WORKDIR /app

# 从 builder 复制 musl 编译的二进制
COPY --from=builder /app/target/x86_64-unknown-linux-musl/release/proxygh ./

RUN chmod +x ./proxygh

EXPOSE 3000

CMD ["./proxygh"]