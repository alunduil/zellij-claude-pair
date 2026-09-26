#!/bin/bash

# SPDX-FileCopyrightText: 2026 Alex Brandt
#
# SPDX-License-Identifier: MIT

# Install the pinned cargo-deny binary into ~/.local/bin. Under GitHub
# Actions, also put that directory on PATH for later steps.

set -euo pipefail

# renovate: datasource=github-releases depName=EmbarkStudios/cargo-deny
CARGO_DENY_VERSION="0.20.2"
CARGO_DENY_NAME="cargo-deny-${CARGO_DENY_VERSION}-x86_64-unknown-linux-musl"
CARGO_DENY_URL="https://github.com/EmbarkStudios/cargo-deny/releases/download/${CARGO_DENY_VERSION}/${CARGO_DENY_NAME}.tar.gz"

temp_dir=$(mktemp -d)
trap 'rm -rf "${temp_dir}"' EXIT

curl -sSfL --retry 5 --retry-delay 2 --retry-all-errors --connect-timeout 10 \
  "${CARGO_DENY_URL}" -o "${temp_dir}/${CARGO_DENY_NAME}.tar.gz"

# The tarball wraps its contents in a target-named directory.
tar -xzf "${temp_dir}/${CARGO_DENY_NAME}.tar.gz" -C "${temp_dir}" --strip-components=1

install_dir="${HOME}/.local/bin"
mkdir -p "${install_dir}"
mv "${temp_dir}/cargo-deny" "${install_dir}/"

if [[ -n "${GITHUB_PATH:-}" ]]; then
  echo "${install_dir}" >>"${GITHUB_PATH}"
fi

"${install_dir}/cargo-deny" --version
