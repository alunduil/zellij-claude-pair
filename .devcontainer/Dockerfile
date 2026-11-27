# SPDX-FileCopyrightText: 2026 Alex Brandt
#
# SPDX-License-Identifier: MIT

# Renovate's minimum release age needs release timestamps. Docker Hub provides
# them, and GHCR doesn't.
FROM jdxcode/mise:2026.11.14@sha256:7178a0bc7eb30d5090f81237fc0a7830f57e52126b9d8df656e24547437a1e53 AS mise

FROM mcr.microsoft.com/devcontainers/base:3.0.14-ubuntu24.04@sha256:88171bb581e19bbb45db77a16a3a5475e2f9ddce025b044c2e5a69a6a1e24fa1

# Even at a pinned version, the mise dev container feature sends GitHub's API an
# unauthenticated request, which the rate limit can reject on a shared runner.
COPY --from=mise /usr/local/bin/mise /usr/local/bin/
