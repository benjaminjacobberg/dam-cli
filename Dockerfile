# Syntax=docker/dockerfile:1
ARG BASE_IMAGE=ubuntu:24.04
FROM ${BASE_IMAGE}

# Stack script - can be preset (minimal, java, nodejs, python, golang) or custom path
ARG STACK_SCRIPT=stacks/minimal.sh

# Base setup
RUN apt-get update && apt-get upgrade -y && \
    apt-get install -y curl ca-certificates zip unzip && \
    apt-get clean && rm -rf /var/lib/apt/lists/*

# Install SDKMAN
RUN curl -s "https://get.sdkman.io" | bash
ENV SDKMAN_DIR="/root/.sdkman"
ENV PATH="${SDKMAN_DIR}/bin:${PATH}"

# Install OpenCode
RUN curl -fsSL https://opencode.ai/install | bash
ENV PATH="/root/.opencode/bin:${PATH}"

# Copy stack scripts
COPY stacks/ /stacks/

# Determine effective stack script path
# If STACK_SCRIPT contains /, treat as custom path; otherwise use presets
RUN if echo "${STACK_SCRIPT}" | grep -q /; then \
        STACK_PATH="${STACK_SCRIPT}"; \
    else \
        STACK_PATH="/stacks/${STACK_SCRIPT}.sh"; \
    fi && \
    if [ -f "${STACK_PATH}" ]; then \
        echo "Loading stack: ${STACK_PATH}" && \
        chmod +x "${STACK_PATH}" && \
        bash "${STACK_PATH}"; \
    else \
        echo "Stack script not found: ${STACK_PATH}" && exit 1; \
    fi

WORKDIR /app
COPY . .

CMD ["/bin/bash"]