FROM rust:1.82

WORKDIR /app

COPY Cargo.toml ./
RUN cargo fetch

COPY . .

EXPOSE 8080
