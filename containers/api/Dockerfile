# 開発環境
FROM rust:1.57.0 as development-stage

WORKDIR /app

ENV USER=our_chat
ENV RUST_BACKTRACE=full

RUN apt-get update \
  # mysql dependencies
  && apt-get install -y libasound2 \
  && apt-get install -y libssl-dev \
  && apt-get install -y gcc \
  && apt-get install -y default-mysql-client \
  # Translations dependencies
  && apt-get install -y gettext \
  # ログインして調査する用
  && apt-get install -y vim \
  && apt-get install -y libxcb-randr0-dev \
  && apt-get install -y libxcb-xtest0-dev \
  && apt-get install -y libxcb-xinerama0-dev \
  && apt-get install -y libxcb-shape0-dev \
  && apt-get install -y libxcb-xkb-dev \
  && apt-get install -y libgconf-2-4 \
  && apt-get install -y libgbm-dev \
  && apt-get install -y libnss3 \
  && apt-get install -y libgl1-mesa-dev \
  && apt-get install -y wget \
  && apt-get install -y gnupg \
  && apt-get install -y musl-tools \
	# Install for xlsxwriter
	&& apt-get install -y clang \
	# Install for printer manipulation
  && apt-get install -y cups \
	# && apt-get install -y libclang-dev \
  && rustup target add x86_64-unknown-linux-musl \
  # cleaning up unused files
  && apt-get purge -y --auto-remove -o APT::AutoRemove::RecommendsImportant=false \
  && rm -rf /var/lib/apt/lists/*

RUN cargo install cargo-watch
RUN cargo install cargo-edit
RUN cargo install diesel_cli --no-default-features --features mysql

COPY . .

COPY ./containers/api/start /start
RUN sed -i 's/\r$//g' /start
RUN chmod +x /start

COPY ./containers/api/local /local
RUN sed -i 's/\r$//g' /local
RUN chmod +x /local

CMD ["/local"]


# ビルド環境
FROM development-stage as build-stage
RUN cargo build --release


# 本番環境
FROM rust:1.43.1-slim-stretch
RUN pwd
RUN pwd
COPY --from=build-stage /app/target/release/build /usr/local/bin/myapp
RUN ls -la
EXPOSE 8088
CMD ["cargo run"]
