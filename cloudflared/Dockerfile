FROM archlinux:latest
# Update and install necessary packages
FROM archlinux:latest
# Update and install necessary packages
RUN pacman -Syu --noconfirm
RUN pacman -S --noconfirm cloudflared
# Expose ports
EXPOSE 8443 8080
# Run Cloudflared when container starts
CMD ["cloudflared"]
