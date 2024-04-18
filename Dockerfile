FROM rust:1.72 as build
WORKDIR /app
COPY . .
# sudo apt install libpq-dev
RUN cargo build --release

FROM rust:1.72
WORKDIR /app
COPY --from=build /app/target/release/proveditor-infrastructure  /usr/src/proveditor-infrastructure
EXPOSE 8081
ENTRYPOINT [ "/usr/src/proveditor-infrastructure" ]