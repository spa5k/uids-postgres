#
# NOTE: you must have the base packaging layer built for this image to work
# you can build this from scratch with `just build-base-pkg-image`
#
# `cargo pgrx init` is run in the base-pkg, so it contains the versions of pg that will be used.
#
FROM ghcr.io/spa5k/uids-postgres/base-pkg:0.1.x-alpine3.18-amd64 AS builder

ARG USER
ENV USER=$USER

ARG PGRX_PG_VERSION=pg16
ENV PGRX_PG_VERSION=$PGRX_PG_VERSION

ARG PKG_PG_VERSION=16.3
ENV PKG_PG_VERSION=$PKG_PG_VERSION

ENV PKG_TARBALL_SUFFIX="-musl"

# Re-run the build with the latest code
WORKDIR /uids
COPY . .
RUN RUSTFLAGS="-Ctarget-feature=-crt-static" just build package

FROM postgres:16-alpine

# NOTE: PGRX_PG_VERSION is defined via base-pkg:pg16.5-alpine3.18-amd64
ARG PGRX_PG_VERSION=pg16
ARG PGIDKIT_VERSION
ARG PGIDKIT_REVISION

# Install packaged uids for system postgres
COPY --from=builder /uids/uids-*-musl.tar.gz /tmp
RUN tar -C /usr/local --strip-components=1 -xvf /tmp/uids-*-musl.tar.gz

LABEL org.opencontainers.image.authors="spa5k"
LABEL org.opencontainers.image.description="A distribution of the base postgres image, with uids pre-installed."
LABEL org.opencontainers.image.documentation="https://github.com/spa5k/uids-postgres#readme"
LABEL org.opencontainers.image.licenses="Apache-2.0"
LABEL org.opencontainers.image.revision=$PGIDKIT_REVISION
LABEL org.opencontainers.image.source="https://github.com/spa5k/uids-postgres"
LABEL org.opencontainers.image.title="Postgres + uids"
LABEL org.opencontainers.image.url="https://github.com/spa5k/uids-postgres"
LABEL org.opencontainers.image.vendor="spa5k"
LABEL org.opencontainers.image.version=v${PGIDKIT_VERSION}