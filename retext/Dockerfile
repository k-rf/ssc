FROM rust:1.52

COPY ./ ./
RUN cargo build --release

ENTRYPOINT [ "cargo" ]
CMD ["run", "--release"]

# [Optional] Uncomment this section to install additional packages.
# RUN apt-get update && export DEBIAN_FRONTEND=noninteractive \
#     && apt-get -y install --no-install-recommends <your-package-list-here>
