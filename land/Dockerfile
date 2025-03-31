# Stage 1: Builder with AUR support
FROM archlinux:latest AS builder

# Install base dependencies
RUN pacman -Syu --noconfirm && \
    pacman -S --noconfirm git base-devel nodejs npm cmake make gcc sudo

# Create non-root builder user and give sudo privileges
RUN useradd -m -U -u 1000 builder && \
    echo "builder ALL=(ALL) NOPASSWD: ALL" >> /etc/sudoers && \
    chown -R builder:builder /home/builder

USER builder
WORKDIR /home/builder

# Install paru as non-root user
RUN git clone https://aur.archlinux.org/paru-bin.git && \
    cd paru-bin && \
    makepkg --noconfirm && \
    sudo pacman -U --noconfirm paru-bin-*.pkg.tar.zst && \
    cd .. && \
    rm -rf paru-bin

# Install quantum-safe dependencies as non-root
RUN paru -S --noconfirm liboqs oqsprovider && \
    sudo pacman -S --noconfirm openssl

# Switch back to root for configuration
USER root

# Configure quantum-safe OpenSSL
RUN mkdir -p /etc/ssl/oqs && \
    echo "openssl_conf = openssl_init" >> /etc/ssl/openssl.cnf && \
    echo "[openssl_init]" >> /etc/ssl/openssl.cnf && \
    echo "providers = provider_sect" >> /etc/ssl/openssl.cnf && \
    echo "[provider_sect]" >> /etc/ssl/openssl.cnf && \
    echo "default = default_sect" >> /etc/ssl/openssl.cnf && \
    echo "oqs = oqs_sect" >> /etc/ssl/openssl.cnf && \
    echo "[default_sect]" >> /etc/ssl/openssl.cnf && \
    echo "activate = 1" >> /etc/ssl/openssl.cnf && \
    echo "[oqs_sect]" >> /etc/ssl/openssl.cnf && \
    echo "activate = 1" >> /etc/ssl/openssl.cnf

# Stage 2: Production image
FROM archlinux:latest

# Copy quantum-safe dependencies
COPY --from=builder /usr/lib/liboqs.so* /usr/lib/
COPY --from=builder /etc/ssl/openssl.cnf /etc/ssl/
COPY --from=builder /usr/lib/ossl-modules/oqsprovider.so /usr/lib/ossl-modules/

# Install runtime dependencies
RUN pacman -Syu --noconfirm && \
    pacman -S --noconfirm nodejs npm openssl && \
    pacman -Scc --noconfirm

# Create the /app directory
RUN mkdir /app
# Create non-root app user with a system UID
RUN useradd -m -r -U -u 999 appuser && \
    chown -R appuser:appuser /app

USER appuser
WORKDIR /app

COPY --chown=appuser:appuser package*.json ./
RUN npm install --production

COPY --chown=appuser:appuser . .
RUN npm run build

EXPOSE 3000

CMD ["npm", "start"]

