# 使用官方 Rust 镜像作为构建环境
FROM rust:1.92.0-bullseye as builder

# 设置工作目录
WORKDIR /app

# 复制 Cargo 文件
COPY Cargo.toml Cargo.lock ./

# 安装依赖
RUN apt-get update && apt-get install -y cmake nasm build-essential pkg-config

# 创建一个虚拟的src目录以允许cargo检查依赖
RUN mkdir src
RUN echo "fn main() { println!(\"Hello, world!\"); }" > src/main.rs

# 复制实际源代码
COPY src ./src

# 构建应用
RUN cargo build --release --locked

# 使用轻量级基础镜像作为运行环境
FROM alpine:3.23

# 安装运行时依赖
RUN apk add --no-cache ca-certificates

# 设置工作目录
WORKDIR /app

# 从构建器复制编译好的二进制文件
COPY --from=builder /app/target/release/proxygh .

# 设置权限
RUN chmod +x proxygh

# 暴露端口
EXPOSE 3000

# 运行应用
CMD ["./proxygh"]