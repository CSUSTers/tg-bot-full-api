FROM alpine AS builder

RUN apk update && apk add --no-cache cmake gcc g++ gperf git make linux-headers musl-dev
RUN apk add --no-cache libressl-dev zlib-static zlib-dev

RUN git clone --depth 1 --shallow-submodules --single-branch --recursive https://github.com/tdlib/telegram-bot-api.git src
RUN --mount=type=cache,target=/src/build cd src/build && \
    cmake -DCMAKE_BUILD_TYPE=Release \
    -DBUILD_SHARED_LIBS=OFF \
    -DOPENSSL_USE_STATIC_LIBS=ON \
    -DZLIB_USE_STATIC_LIBS=ON \
    -DTELEGRAM_BOT_API_ENABLE_LTO=ON \
    -DCMAKE_C_FLAGS="-O2" \
    -DCMAKE_CXX_FLAGS="-O2" \
    -DCMAKE_EXE_LINKER_FLAGS="-static -static-libstdc++ -static-libgcc" .. && \
    make -j $(nproc) && mv telegram-bot-api /


FROM rust:1.86-alpine AS builder2

RUN apk add --no-cache musl-dev libressl-dev

WORKDIR /src
COPY . .
RUN --mount=type=cache,target=/src/target \
  cargo build --release && \
  cp target/release/tg-bot-full-api /tg-bot-full-api


FROM alpine

# RUN apk update && apk add --no-cache openssl zlib libstdc++
COPY --from=builder /telegram-bot-api/ /
COPY --from=builder2 /tg-bot-full-api /

EXPOSE 3000

ENTRYPOINT [ "/tg-bot-full-api" ]
