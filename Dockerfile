FROM rust:latest as builder
WORKDIR /usr/src/todo-bot-1
COPY . .
RUN ls -la
RUN cargo build --release

FROM debian:bullseye-slim
COPY --from=builder /usr/src/todo-bot-1/target/release/todo-bot-1 /usr/local/bin/todo-bot-1
RUN apt update && apt install ca-certificates -y
RUN apt install libpq-dev -y
CMD ["todo-bot-1"]

