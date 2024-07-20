FROM mcr.microsoft.com/devcontainers/rust:1-1-bullseye

# Install debian packages for developement
RUN apt-get update && apt-get -y install \
    cmake \
    ninja-build \
    protobuf-compiler \
    libprotoc-dev \
    && rm -rf /var/lib/apt/lists/*

# Install binstall
RUN curl -L --proto '=https' --tlsv1.2 -sSf https://raw.githubusercontent.com/cargo-bins/cargo-binstall/main/install-from-binstall-release.sh | bash
# Install cargo-fuzz
RUN cargo binstall -y cargo-fuzz
# Install nightly compiler
RUN rustup install nightly