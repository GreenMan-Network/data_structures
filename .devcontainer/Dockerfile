FROM rust:latest

# Install additional tools to optimize development and builds
RUN apt-get update && export DEBIAN_FRONTEND=noninteractive \
    && apt-get -y install clang lld cmake libssl-dev pkg-config build-essential git curl \
    && apt-get autoremove -y && apt-get clean -y

# Ensure rustfmt and clippy are installed
RUN rustup component add rustfmt \
    && rustup component add clippy

# Install cargo-watch to automatically recompile on file changes
RUN cargo install cargo-watch

# Install cargo-audit to check for vulnerabilities in dependencies
RUN cargo install cargo-audit

# Install cargo-nextest for faster parallel test execution
RUN cargo install cargo-nextest

# Install cargo-edit to easily manage dependencies (add/remove/update)
RUN cargo install cargo-edit

# Configure .bashrc for root to enable colors
RUN echo "if [ -x /usr/bin/dircolors ]; then\n\
    eval \"\$(dircolors -b)\"\n\
    alias ls='ls --color=auto'\n\
    alias grep='grep --color=auto'\n\
    alias fgrep='fgrep --color=auto'\n\
    alias egrep='egrep --color=auto'\n\
fi\n\
PS1='\\[\\033[01;32m\\]\\u@\\h \\[\\033[01;34m\\]\\w \\$\\[\\033[00m\\] '\n\
export TERM=xterm-256color" >> /root/.bashrc

# Set default working directory
WORKDIR /app

# Default command when starting the container (can be modified if needed)
CMD [ "bash" ]
