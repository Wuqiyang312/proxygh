# 使用官方 Rust 镜像作为构建环境
FROM rust:1.80 as builder

# 设置工作目录
WORKDIR /app

# 复制项目文件
COPY Cargo.toml Cargo.lock ./
COPY src ./src

# 预编译依赖
RUN cargo build --release
RUN rm src/main.rs  # 删除默认的main.rs，因为我们要重新编译

# 复制源代码
COPY src ./src

# 构建发布版本
RUN cargo build --release

# 使用轻量级基础镜像作为运行环境
FROM debian:bookworm-slim

# 安装运行时依赖
RUN apt-get update && apt-get install -y \
    ca-certificates \
    && rm -rf /var/lib/apt/lists/*

# 设置工作目录
WORKDIR /app

# 从构建器复制编译好的二进制文件
COPY --from=builder /app/target/release/proxygh .

# 暴露端口
EXPOSE 3000

# 运行应用
CMD ["./proxygh"]