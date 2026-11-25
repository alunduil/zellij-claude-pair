# SPDX-FileCopyrightText: 2026 Alex Brandt
#
# SPDX-License-Identifier: MIT

# Docker Hub gives Renovate the release timestamps that its minimum release age
# needs; GHCR gives none.
FROM jdxcode/mise:2026.11.12@sha256:991e2a550518ec5cae6f25adbc862ec0253ff714ee6e5e6bf328058a7324a041 AS mise

FROM mcr.microsoft.com/devcontainers/base:3.0.14-ubuntu24.04@sha256:88171bb581e19bbb45db77a16a3a5475e2f9ddce025b044c2e5a69a6a1e24fa1

# The mise dev container feature calls GitHub's API without a token, even for a
# pinned version, so a shared runner's rate limit fails the build.
COPY --from=mise /usr/local/bin/mise /usr/local/bin/
