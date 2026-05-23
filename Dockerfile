FROM rust:latest

COPY ./ /app
WORKDIR /app

ENTRYPOINT ["bash"]
