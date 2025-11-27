FROM archlinux:latest

# Install dependencies
RUN pacman -Syu --noconfirm && \
    pacman -S --noconfirm rust perl clang git cmake base-devel openssl

# Clone Pingora with specific commit
RUN git clone https://github.com/cloudflare/pingora.git && \
    cd pingora && \
    git checkout 7e2c107  # Known stable commit

# Build with backtrace support
WORKDIR /pingora
ENV RUST_BACKTRACE=1
RUN cargo build --release --features=full

# Copy config
COPY pingora.conf /pingora/conf/pingora.conf

# Run with logging
CMD ["cargo", "run", "--release", "--", "-c", "conf/pingora.conf"]

