FROM docker.io/fedora:37

RUN dnf update -y

RUN dnf install -y gdal gdal-devel hdf5 hdf5-devel pkg-config openssl-devel

COPY --from=build_image ./lp/target/release/sat_dl /sat_dl/

WORKDIR /sat_dl
