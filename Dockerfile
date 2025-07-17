# === Stage 1: build ===
FROM rust:1.87 AS builder

# Zet de working dir gelijk aan je projectnaam
WORKDIR /usr/src/spacetime_lite

# Kopieer alleen de Cargo-manifesten om dependencies te cachen
COPY Cargo.toml Cargo.lock ./

# Dummy src/main.rs aanmaken zodat cargo fetch niet klaagt
RUN mkdir src && \
    echo 'fn main() { println!("dummy"); }' > src/main.rs

# Prefetch dependencies (snellere rebuilds bij code-wijzigingen)
RUN cargo fetch

# Kopieer de rest van de broncode
RUN rm src/main.rs
COPY . .

# Build in release-modus
RUN cargo build --release

# === Stage 2: runtime ===
FROM debian:bookworm-slim

# Certificaten voor HTTPS-client calls
RUN apt-get update \
 && apt-get install -y --no-install-recommends ca-certificates \
 && rm -rf /var/lib/apt/lists/*

# Zet een werkmap voor de runtime (optioneel)
WORKDIR /usr/local/bin

# Kopieer de gecompileerde binary over
COPY --from=builder /usr/src/spacetime_lite/target/release/spacetime_lite .

# Kopieer de static folder
COPY --from=builder /usr/src/spacetime_lite/static ./static

# Documenteer dat je port 8080 gebruikt
EXPOSE 8080

# Stel de entrypoint in
ENTRYPOINT ["./spacetime_lite"]
