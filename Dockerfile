FROM rust:1.72
COPY ./target/release/proveditor-infrastructure /usr/src/rust-infra-service
EXPOSE 8081
ENTRYPOINT [ "/usr/src/rust-infra-service" ]