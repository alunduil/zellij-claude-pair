# SPDX-FileCopyrightText: 2026 Alex Brandt
#
# SPDX-License-Identifier: MIT

# Renovate's minimum release age needs release timestamps. Docker Hub provides
# them, and GHCR doesn't.
FROM jdxcode/mise:2026.11.13@sha256:06a4e5b507ac24d0279d6d2fc1f0849cb54fd0c1eb305a93db028398364b45f9 AS mise

FROM mcr.microsoft.com/devcontainers/base:3.0.14-ubuntu24.04@sha256:88171bb581e19bbb45db77a16a3a5475e2f9ddce025b044c2e5a69a6a1e24fa1

# Even at a pinned version, the mise dev container feature sends GitHub's API an
# unauthenticated request, which the rate limit can reject on a shared runner.
COPY --from=mise /usr/local/bin/mise /usr/local/bin/
