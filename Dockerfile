FROM messense/rust-musl-cross:x86_64-musl as chef

RUN cargo install cargo-chef
WORKDIR /rosatelyon

# --
FROM chef AS planner

COPY . .
RUN cargo chef prepare --recipe-path recipe.json

# --
FROM chef AS builder

COPY --from=planner /rosatelyon/recipe.json recipe.json
RUN cargo chef cook --release --target x86_64-unknown-linux-musl --recipe-path recipe.json
COPY . .

RUN cargo build --release --target x86_64-unknown-linux-musl

# --
FROM scratch

COPY --from=builder /rosatelyon/target/x86_64-unknown-linux-musl/release/rosatelyon /rosatelyon

ENTRYPOINT ["/rosatelyon"]
EXPOSE 3000
