FROM docker.io/fedora:37

COPY --from=build_image ./target/release/map_generation /map_generation/

WORKDIR /map_generation
