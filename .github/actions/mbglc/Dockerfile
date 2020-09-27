FROM ubuntu:19.10

ENV TZ=Europe/Copenhagen
RUN ln -snf /usr/share/zoneinfo/$TZ /etc/localtime && echo $TZ > /etc/timezone

RUN apt-get update && \
    apt-get install -y build-essential libgl-dev cmake libcurl4-gnutls-dev \
    libjpeg-dev libpng-dev zlib1g-dev pkg-config libglfw3-dev libuv1-dev llvm clang

COPY entrypoint.sh /usr/bin/entrypoint.sh

RUN chmod a+x /usr/bin/entrypoint.sh


ENTRYPOINT [ "/usr/bin/entrypoint.sh" ]
