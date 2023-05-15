FROM docker.io/fedora:37

RUN dnf update -y

RUN dnf install -y rust cargo gdal gdal-devel hdf5 hdf5-devel pkg-config openssl-devel

WORKDIR /lp

COPY . .

RUN cargo build --release --bin map_generation

RUN cargo build --release --bin sat_dl
