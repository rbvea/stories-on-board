FROM rustlang/rust:nightly

RUN mkdir -p /app
COPY . /app
WORKDIR /app
RUN cargo build --release
ENV ROCKET_ENV=prod
ENV ROCKET_ADDRESS=0.0.0.0
CMD ROCKET_PORT=$PORT ./target/release/stories-on-board
