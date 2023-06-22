FROM rust as build

WORKDIR /ims

COPY . .

RUN cargo build --bin ims --release


FROM rust:slim
COPY --from=build /ims/target/release/ims .

CMD ["./ims"]

# love you 3000 <3