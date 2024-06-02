FROM ubuntu:jammy
ARG DEBIAN_FRONTEND=noninteractive

# install dependencies
RUN set -eux; \
    apt update; \
    apt upgrade -y; \
    apt install -y --no-install-recommends \
        ca-certificates gcc libc6-dev pkg-config libssl-dev wget git zsh; \
    apt autoremove -y; \
    apt clean; \
    rm -rf /var/lib/apt/lists/*

# create user
ARG USERNAME=dev
RUN set -eux; \
    useradd --create-home --user-group --no-log-init "$USERNAME"; \
    chown -R "$USERNAME:$USERNAME" "/home/$USERNAME"
USER "$USERNAME"

# install rust
RUN set -eux; \
    wget -qO - https://sh.rustup.rs | sh -s -- -y

# entrypoint
CMD ["sleep", "infinity"]