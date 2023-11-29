FROM mcr.microsoft.com/devcontainers/base:ubuntu-22.04

SHELL ["/bin/bash", "--login", "-c"]

# Install rust
RUN apt-get update && export DEBIAN_FRONTEND=noninteractive \
    && apt-get -y install --no-install-recommends curl build-essential

USER vscode
RUN curl https://sh.rustup.rs -sSf | bash -s -- -y
ENV PATH="/home/vscode/.cargo/bin:${PATH}"

USER root

# nvm environment variables
ENV NVM_DIR /usr/local/nvm
ENV NODE_VERSION 21.1.0
RUN mkdir -p $NVM_DIR

# install nvm
# https://github.com/creationix/nvm#install-script
RUN curl --silent -o- https://raw.githubusercontent.com/creationix/nvm/v0.39.5/install.sh | bash
RUN source $NVM_DIR/nvm.sh \
    && nvm install $NODE_VERSION \
    && nvm alias default $NODE_VERSION \
    && nvm use default

# add node and npm to path so the commands are available
ENV NODE_PATH $NVM_DIR/v$NODE_VERSION/lib/node_modules
ENV PATH $NVM_DIR/versions/node/v$NODE_VERSION/bin:$PATH

# Install git
RUN apt-get update && export DEBIAN_FRONTEND=noninteractive \
    && apt-get -y install --no-install-recommends git

# Install tiny-git-helper
RUN npm install -g tiny-git-helper
# Add tgh config folder
ADD .devcontainer/.helper-config /home/vscode/.helper-config

USER root
RUN chown -R vscode:vscode /home/vscode/.helper-config