FROM rust:1.81

WORKDIR /app

# Install required system dependencies
RUN apt-get update && apt-get install -y \
    pkg-config \
    libssl-dev \
    && rm -rf /var/lib/apt/lists/*

# Copy all files
COPY . .

# Build and run the application
RUN cargo build --release && \
    cp target/release/backend /usr/local/bin/ && \
    chmod +x /usr/local/bin/backend

EXPOSE 8000

CMD ["/usr/local/bin/backend"] 