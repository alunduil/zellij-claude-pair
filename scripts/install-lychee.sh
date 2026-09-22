#!/bin/bash

# SPDX-FileCopyrightText: 2026 Alex Brandt
#
# SPDX-License-Identifier: MIT

# Install the pinned lychee binary into ~/.local/bin. Under GitHub Actions,
# also put that directory on PATH for later steps.

set -euo pipefail

# Upstream tags are "lychee-vX.Y.Z"; extractVersion strips the prefix.
# renovate: datasource=github-releases depName=lycheeverse/lychee extractVersion=^lychee-(?<version>v.+)$
LYCHEE_VERSION="v0.24.2"
# The GNU build requires a newer GLIBC than some hosts ship; the static musl
# build runs anywhere.
LYCHEE_ARCHIVE="lychee-x86_64-unknown-linux-musl.tar.gz"
LYCHEE_URL="https://github.com/lycheeverse/lychee/releases/download/lychee-${LYCHEE_VERSION}/${LYCHEE_ARCHIVE}"

temp_dir=$(mktemp -d)
trap 'rm -rf "${temp_dir}"' EXIT

curl -sSfL --retry 5 --retry-delay 2 --retry-all-errors --connect-timeout 10 \
  "${LYCHEE_URL}" -o "${temp_dir}/${LYCHEE_ARCHIVE}"

# The tarball wraps its contents in a target-named directory.
tar -xzf "${temp_dir}/${LYCHEE_ARCHIVE}" -C "${temp_dir}" --strip-components=1

install_dir="${HOME}/.local/bin"
mkdir -p "${install_dir}"
mv "${temp_dir}/lychee" "${install_dir}/"

if [[ -n "${GITHUB_PATH:-}" ]]; then
  echo "${install_dir}" >>"${GITHUB_PATH}"
fi

"${install_dir}/lychee" --version
