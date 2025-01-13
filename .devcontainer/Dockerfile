FROM ubuntu:noble
ARG USERNAME=dev
ARG REPO=/ws

ENV CRSH_USERNAME="$USERNAME" CRSH_REPO="$REPO"

# install mold and clang for faster linking, zsh, sudo and git for convenience
ARG DEBIAN_FRONTEND=noninteractive
RUN set -eux; apt update; apt full-upgrade -y; \
    apt install -y ca-certificates libc6-dev pkg-config libssl-dev \
        mold clang zsh sudo git wget; \
    apt autoremove -y; apt clean; rm -rf /var/lib/apt/lists/*

# delete default user on new ubuntu images
RUN set -eux; \
    grep ubuntu /etc/passwd && \
    touch /var/mail/ubuntu && \
    chown ubuntu /var/mail/ubuntu && \
    userdel -r ubuntu

# create non-root sudo user
RUN set -eux; \
    useradd --create-home --user-group --no-log-init "$CRSH_USERNAME"; \
    echo "$USERNAME ALL=(root) NOPASSWD:ALL" > "/etc/sudoers.d/$CRSH_USERNAME"; \
    chmod 0440 "/etc/sudoers.d/$CRSH_USERNAME"

# add cargo config for multithreaded frontend and use mold linker
COPY fast_cargo_config.toml "/home/$CRSH_USERNAME/.cargo/config.toml"

# create workspace dir and fix ownership
RUN set -eux; \
    mkdir -p "$CRSH_REPO"; \
    chown -R "$CRSH_USERNAME:$CRSH_USERNAME" "$CRSH_REPO" "/home/$CRSH_USERNAME"
WORKDIR "$CRSH_REPO"

# change user
USER "$CRSH_USERNAME"

# install rust
RUN set -eux; wget -qO - https://sh.rustup.rs | sh -s -- -y --default-toolchain nightly

# add local bin to path
ENV PATH="/home/$CRSH_USERNAME/.local/bin:/home/$CRSH_USERNAME/.cargo/bin:$PATH"

# replace entrypoint
CMD sleep infinity