FROM rust:1.52 as build

COPY ./ ./
RUN cargo build --release

FROM scratch
COPY --from=build ./target/release/server ./bin/server
ENV RUST_LOG=info

EXPOSE 8080

ENTRYPOINT [ "/bin/server" ]

# [Optional] Uncomment this section to install additional packages.
# RUN apt-get update && export DEBIAN_FRONTEND=noninteractive \
#     && apt-get -y install --no-install-recommends <your-package-list-here>
