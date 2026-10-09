# syntax=docker/dockerfile:1

ARG IMAGE_VARIANT
ARG RUST_VERSION
ARG UV_VERSION


FROM ghcr.io/astral-sh/uv:${UV_VERSION} AS uv_img


FROM rust:${RUST_VERSION}-${IMAGE_VARIANT} AS base
ENV LANG=C.UTF-8 \
    LC_ALL=C.UTF-8 \
    PATH="/usr/local/cargo/bin:$PATH"


FROM base AS binstall
RUN cargo install --locked cargo-binstall
RUN cargo binstall --locked -y just just-lsp cargo-deny cargo-nextest


FROM base AS final
ARG DEBIAN_FRONTEND=noninteractive
RUN apt-get update && apt-get install --no-install-recommends -y \
    build-essential pkg-config libssl-dev zsh sudo git wget curl gnupg && \
    rm -rf /var/lib/apt/lists/*
ARG USERNAME
ARG WORKSPACE
RUN useradd --create-home --user-group --no-log-init "$USERNAME" && \
    echo "$USERNAME ALL=(root) NOPASSWD:ALL" > "/etc/sudoers.d/$USERNAME" && \
    chmod 0440 "/etc/sudoers.d/$USERNAME" && \
    mkdir -p "$WORKSPACE" && \
    chown -R "$USERNAME:$USERNAME" "$WORKSPACE" "/home/$USERNAME"
WORKDIR ${WORKSPACE}
USER ${USERNAME}
RUN rustup component add clippy rustfmt
COPY --from=binstall --chown="$USERNAME:$USERNAME" /usr/local/cargo/ /usr/local/cargo/
COPY --from=uv_img --chown="$USERNAME:$USERNAME" /uv /uvx /usr/local/bin/
ENV UV_LINK_MODE=copy \
    UV_PYTHON_PREFERENCE=managed \
    UV_COMPILE_BYTECODE=1 \
    PATH="$HOME/.local/bin:$PATH"
ARG PYTHON_VERSION
RUN uv python install ${PYTHON_VERSION} && \
    uv tool install ruff && \
    uv tool install ty
CMD sleep infinity
