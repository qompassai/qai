FROM nginx:latest

# Create the certs directory
RUN mkdir -p /etc/nginx/certs

# Copy your Nginx configuration
COPY nginx.conf /etc/nginx/nginx.conf

# Copy certificates
COPY certs /etc/nginx/certs

# Expose ports
EXPOSE 8080 8443

# Run Nginx when the container starts
CMD ["nginx", "-g", "daemon off;"]

