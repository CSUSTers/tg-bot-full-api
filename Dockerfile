FROM alpine AS builder

RUN apk add --no-cache cmake openssl-dev gcc g++ gperf zlib-dev git make linux-headers musl

RUN git clone --depth 1 --recursive https://github.com/tdlib/telegram-bot-api.git
RUN CC=musl-cc CXX=musl-c++ cd telegram-bot-api && mkdir build && cd build && \
    cmake -DCMAKE_BUILD_TYPE=Debug .. && make -j 8


FROM rust:1.83-alpine AS builder2

RUN apk add --no-cache musl-dev

WORKDIR /tg-bot-full-api

COPY . .

RUN cargo build --release


FROM alpine

RUN apk add --no-cache openssl zlib libstdc++

COPY --from=builder /telegram-bot-api/build/telegram-bot-api /

COPY --from=builder2 /tg-bot-full-api/target/release/tg-bot-full-api /

EXPOSE 3000

ENTRYPOINT [ "/tg-bot-full-api" ]
