FROM rust:1.62.1-alpine3.16

WORKDIR /app
COPY . .
RUN apk update && apk add musl-dev
RUN cargo install --path . && cp /usr/local/cargo/bin/space /usr/local/bin/space

EXPOSE 8000

CMD ["space"]