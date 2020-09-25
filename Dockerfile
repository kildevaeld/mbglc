FROM ubuntu:latest

ENV TZ=Europe/Copenhagen
RUN ln -snf /usr/share/zoneinfo/$TZ /etc/localtime && echo $TZ > /etc/timezone

RUN apt-get update && \
    apt-get install -y build-essential libgl-dev cmake libcurl4-gnutls-dev \
    libjpeg-dev libpng-dev zlib1g-dev pkg-config libglfw3-dev libuv1-dev llvm clang



WORKDIR /workspace

COPY . /workspace/

RUN cd /workspace && \
    rm -rf build-linux; mkdir build-linux; cd build-linux && \
    cmake -DCMAKE_BUILD_TYPE=Release .. && \
    cmake --build . --config Release -j 4 && \
    cmake --build . --config Release --target package

FROM ubuntu:latest

ENV TZ=Europe/Copenhagen
RUN ln -snf /usr/share/zoneinfo/$TZ /etc/localtime && echo $TZ > /etc/timezone

COPY --from=0 /workspace/build/*.deb /

RUN dpkg -i /*.deb