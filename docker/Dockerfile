FROM postgres:16
RUN apt-get update && apt-get upgrade -y

ENV build_deps ca-certificates \
  git \
  build-essential \
  libpq-dev \
  postgresql-server-dev-14 \
  curl \
  libreadline6-dev \
  zlib1g-dev


RUN apt-get install -y --no-install-recommends $build_deps pkg-config cmake

WORKDIR /home/spa5k

ENV HOME=/home/spa5k \
  PATH=/home/spa5k/.cargo/bin:$PATH
RUN chown postgres:postgres /home/spark
USER postgres

RUN \
  curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y --no-modify-path --profile minimal --default-toolchain nightly && \
  rustup --version && \
  rustc --version && \
  cargo --version

# PGX
RUN cargo install cargo-pgrx

RUN cargo pgx init --pg16 $(which pg_config)

USER root


RUN mkdir -p /tmp/build \
  && cd /tmp/build \
  && git clone https://github.com/spa5k/uids-postgres \
  && cd uids-postgres \
  && cargo pgx install

RUN rm -fr /tmp/build \
  && apt-get clean \
  && apt-get autoremove -y $build_deps
