FROM rust:1.67 AS build
COPY . .

FROM alpine:3.16.0 AS runtime
COPY --from=build /usr/local/cargo/bin/fibbot /usr/local/bin/fibbot

FROM runtime as action
COPY entrypoint.sh /entrypoint.sh

ENTRYPOINT [ /entrypoint.sh ]
